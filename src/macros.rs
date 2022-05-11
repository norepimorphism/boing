// SPDX-License-Identifier: MPL-2.0

// This macro is commonly used such that the returned `CString` is immediately dropped after a pointer to its buffer is passed to a *libui-ng* function. This seems like a typical use-after-free, but it's actually OK! As far as I can tell, *libui-ng* (or the underlying OS UI libraries) `strdup` string arguments.

/// Creates a new `CString`, returning from the current scope if an error occurs during the
/// conversion.
macro_rules! make_cstring {
    ($contents:expr $(,)?) => {
        std::ffi::CString::new($contents).map_err($crate::Error::ConvertRustString)?
    };
}

/// Defines a new control type.
///
/// Note that this function should not and cannot be used on all widgets, and in particular,
/// [`crate::Menu`] and [`crate::MenuItem`].
macro_rules! def_subcontrol {
    (
        docs: $docs:literal,
        ty: $ty:ident,
        handle: $ptr_ty:ident
        $(
            ,
            cb_fns: [
                $($cb:ident<$lt:lifetime>() $(-> $out:ty)?),* $(,)?
            ]
        )? $(,)?
    ) => {
        impl$(<$($lt),*>)? $ty$(<$($lt),*>)? {
            pub(crate) fn new(ptr: *mut $ptr_ty) -> Self {
                Self::from_control(Control::new(ptr.cast()))
            }

            pub(crate) fn from_control(control: Control) -> Self {
                Self {
                    inner: control,
                    $(
                        $(
                            $cb: None
                        ),*
                    )?
                }
            }
        }

        #[doc = indoc::indoc!($docs)]
        pub struct $ty$(<$($lt),*>)? {
            inner: $crate::Control,
            $(
                $(
                    $cb: Option<Box<dyn $lt + FnMut(&mut Self) $(-> $out)?>>
                ),*
            )?
        }

        impl$(<$($lt),*>)? $ty$(<$($lt),*>)? {
            /// A handle to the underlying *libui-ng* object.
            pub fn as_ptr(&self) -> *mut $ptr_ty {
                self.inner.as_ptr().cast()
            }
        }

        impl$(<$($lt),*>)? std::ops::Deref for $ty$(<$($lt),*>)? {
            type Target = Control;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl$(<$($lt),*>)? std::ops::DerefMut for $ty$(<$($lt),*>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };
}

macro_rules! call_fallible_libui_fn {
    ($fn:ident( $($arg:expr),* $(,)? )) => {
        unsafe { $fn( $($arg),* ).as_mut() }
            .ok_or($crate::Error::LibuiFn { name: stringify!($fn), cause: None })
    };
}

macro_rules! call_libui_new_fn {
    (
        ui: $ui:expr,
        fn: $fn:ident( $($arg:expr),* $(,)? ) -> $out_ty:ident,
    ) => {
        call_fallible_libui_fn!( $fn($($arg),*) )
            .map(|ptr| $out_ty::new(ptr))
    };
}

macro_rules! bind_callback_fn {
    (
        docs: $docs:literal,
        self: {
            ty: $self_ty:tt<$cb_lt:lifetime>,
            handle: $self_handle_ty:ident,
            fn: $fn:ident(),
            cb: {
                sig: $user_cb:ident -> $user_cb_out:ty
                $(, map: $map_user_cb:expr )? $(,)?
            } $(,)?
        },
        libui: {
            fn: $libui_fn:ident($(, $($libui_arg:expr),* )?),
            cb: {
                sig: ($($cb_arg:ty),*) -> $libui_cb_out:ty $(,)?,
            } $(,)?
        } $(,)?
    ) => {
        // Wow, callbacks are complicated!

        #[doc = indoc::indoc!($docs)]
        pub fn $fn<F>(&mut self, $user_cb: F)
        where
            F: $cb_lt + FnMut(&mut Self) -> $user_cb_out,
        {
            /// A trampoline function to the user-set callback.
            unsafe extern "C" fn trampoline(
                handle: *mut libui_ng_sys::$self_handle_ty,
                $(_: $cb_arg,)*
                user_cb: *mut std::os::raw::c_void,
            ) -> $libui_cb_out {
                // Ensure nothing wonky has happened in the meantime.
                debug_assert!(!handle.is_null());
                debug_assert!(!user_cb.is_null());

                let user_cb: &mut Option<&mut dyn FnMut(&mut $self_ty) -> $user_cb_out> = &mut *user_cb.cast();

                // SAFETY:
                //
                // The [`Option` docs] state that transmutation is valid from `Some(T)` to `T`. In
                // this case, the `Option` is definitely `Some` (unless something wonky happened!),
                // so this should be safe.
                //
                // [`Option` docs]: https://doc.rust-lang.org/std/option/index.html#representation
                debug_assert!(user_cb.is_some());
                let user_cb: &mut &mut dyn FnMut(&mut $self_ty) -> $user_cb_out = std::mem::transmute(user_cb);

                let mut handle = std::mem::ManuallyDrop::new(<$self_ty>::new(handle));

                let result = (user_cb)(&mut handle);
                $(
                    let result = $map_user_cb(result);
                )?

                result
            }

            self.$fn = Some(Box::new($user_cb));

            unsafe {
                $libui_fn(
                    self.as_ptr(),
                    $(
                        $($libui_arg),*
                    )?
                    Some(trampoline),
                    std::ptr::addr_of_mut!(self.$fn).cast(),
                );
            }
        }
    };
}

macro_rules! bind_add_child_fn {
    (
        docs: $docs:literal,
        self: {
            fn: $fn:ident<$ui_lt:lifetime>,
            child: $child:ident $(,)?
        },
        libui: {
            fn: $libui_fn:ident $(,)?
        } $(,)?
    ) => {
        #[doc = indoc::indoc!($docs)]
        pub fn $fn(&self, $child: &mut impl std::ops::DerefMut<Target = Control>) {
            // Inform the child control that it should not destroy itself as *libui-ng* will take
            // care of that for us.
            $child.make_child();

            unsafe { $libui_fn(self.as_ptr(), $child.as_ptr()) };
        }
    };
}

macro_rules! bind_text_fn {
    (
        docs: $docs:literal,
        self: {
            fn: $fn:ident,
            raw_fn: $raw_fn:ident,
            as_ptr_fn: $as_ptr_fn:ident $(,)?
        },
        libui: {
            fn: $libui_fn:ident($($arg:expr),* $(,)?) $(,)?
        } $(,)?
    ) => {
        #[doc = indoc::indoc!($docs)]
        pub fn $fn(&self) -> String {
            let ptr = self.$as_ptr_fn();
            let string = unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into() };

            // Now that the contents of `ptr` have been copied into `string`, we can safely free the
            // original string.
            unsafe { uiFreeText(ptr) };

            string
        }

        #[doc = concat!("The lossless yet fallible version of [`Self::", stringify!($fn), "`].")]
        pub fn $raw_fn(&self) -> Result<&str, $crate::Error> {
            unsafe { std::ffi::CStr::from_ptr(self.$as_ptr_fn()) }
                .to_str()
                .map_err($crate::Error::ConvertCString)
        }

        pub fn $as_ptr_fn(&self) -> *mut std::os::raw::c_char {
            unsafe {
                $libui_fn(
                    $(
                        $($arg),*
                    )?
                    self.as_ptr(),
                )
            }
        }
    };
}

macro_rules! bind_set_text_fn {
    (docs: $docs:literal, $fn:ident, $arg:ident, $libui_fn:ident $(,)?) => {
        #[doc = indoc::indoc!($docs)]
        pub fn $fn(&self, $arg: impl AsRef<str>) -> Result<(), $crate::Error> {
            // Normally, this is a bad idea: `$arg` is a `CString` that will be dropped at the end
            // of this scope, but its pointer is being passed to a C function that may, in theory,
            // refer to it indefinitiely long. However, as far as I know, *libui-ng* `strdup`s all
            // string arguments before using them, so this should be safe.
            let $arg = make_cstring!($arg.as_ref());

            unsafe { $libui_fn(self.as_ptr(), $arg.as_ptr()) };

            Ok(())
        }
    };
}

/// Defines a binding to a *libui-ng-sys* function that returns `bool`.
macro_rules! bind_bool_fn {
    (docs: $docs:literal, $fn:ident, $libui_fn:ident $(,)?) => {
        #[doc = indoc::indoc!($docs)]
        pub fn $fn(&self) -> bool {
            let result = unsafe { $libui_fn(self.as_ptr()) };

            // A boolean should be 0 or 1; if not, something may be awry.
            debug_assert!((result == 0) || (result == 1));

            // We could test `result == 1`, but many C programs test booleans with `if(_)`---which
            // is shorthand for `if(_ != 0)`---so we will replicate that functionality here.
            result != 0
        }
    };
}

/// Defines a binding to a *libui-ng-sys* function that accepts a `bool`.
macro_rules! bind_set_bool_fn {
    (docs: $docs:literal, $fn:ident, $libui_fn:ident $(,)?) => {
        #[doc = indoc::indoc!($docs)]
        pub fn $fn(&self, value: bool) {
            unsafe { $libui_fn(self.as_ptr(), value.into()) };
        }
    };
}
