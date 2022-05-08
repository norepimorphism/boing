// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.


macro_rules! ui {
    () => {
        $crate::Ui
    };
}

macro_rules! make_cstring {
    ($contents:expr $(,)?) => {
        std::ffi::CString::new($contents).map_err($crate::Error::ConvertRustString)?
    };
}

macro_rules! def_subcontrol {
    (
        ty: $ty:ident,
        handle: $ptr_ty:ident
        $(
            , cb_fns: [
                $($cb:ident() $(-> $out:ty)?),* $(,)?
            ]
        )? $(,)?
    ) => {
        pub struct $ty
        {
            inner: $crate::Control,
            $(
                $(
                    $cb: Box<dyn FnMut() $(-> $out)?>
                ),*
            )?
        }

        impl $ty {
            unsafe fn from_ptr(ptr: *mut $ptr_ty) -> Self {
                debug_assert!(!ptr.is_null());

                Self::from_control(Control::from_ptr(ptr.cast()))
            }

            pub(crate) unsafe fn from_control(control: Control) -> Self {
                Self {
                    inner: control,
                    $(
                        $(
                            $cb: Box::new(|| { Default::default() })
                        ),*
                    )?
                }
            }

            pub fn as_ptr(&self) -> *mut $ptr_ty {
                self.inner.as_ptr().cast()
            }
        }

        impl std::ops::Deref for $ty {
            type Target = Control;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl std::ops::DerefMut for $ty {
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
        alloc: $alloc:ident,
        fn: $fn:ident( $($arg:expr),* $(,)? ) -> $out_ty:ident,
    ) => {
        call_fallible_libui_fn!( $fn($($arg),*) )
            .map(|ptr| unsafe {
                $ui.$alloc($out_ty::from_ptr(ptr))
            })
    };
}

macro_rules! bind_callback_fn {
    (
        docs: $docs:literal,
        self: {
            ty: $self_ty:tt,
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

        #[doc = $docs]
        pub fn $fn<'a, F>(&'a mut self, ui: &Ui, $user_cb: F)
        where
            F: 'a + FnMut() -> $user_cb_out,
        {
            /// A trampoline function to the user-set callback.
            unsafe extern "C" fn trampoline<F>(
                handle: *mut libui_ng_sys::$self_handle_ty,
                $(_: $cb_arg,)*
                user_cb: *mut std::os::raw::c_void,
            ) -> $libui_cb_out
            where
                F: FnMut() -> $user_cb_out,
            {
                // Ensure nothing wonky has happened in the meantime.
                debug_assert!(!handle.is_null());
                debug_assert!(!user_cb.is_null());

                let user_cb: &mut Box<F> = &mut *user_cb.cast();
                let result = (user_cb)();
                $(
                    let result = $map_user_cb(result);
                )?

                result
            }

            self.$fn = Box::new($user_cb);

            unsafe {
                $libui_fn(
                    self.as_ptr(),
                    $(
                        $($libui_arg),*
                    )?
                    Some(trampoline::<F>),
                    std::ptr::addr_of_mut!(self.$fn).cast(),
                );
            }
        }
    };
}

macro_rules! bind_add_child_fn {
    (
        docs: $docs:literal,
        $fn:ident,
        $child:ident,
        $libui_fn:ident $(,)?
    ) => {
        #[doc = $docs]
        pub fn $fn(&self, $child: &mut impl std::ops::DerefMut<Target = Control>) {
            let $child = std::mem::ManuallyDrop::new($child);
            unsafe { $libui_fn(self.as_ptr(), $child.as_ptr()) };
        }
    };
}

macro_rules! bind_text_fn {
    (
        docs: $docs:literal,
        $fn:ident,
        $raw_fn:ident,
        $fn_ptr:ident,
        $libui_fn:expr
        $(, $($arg:expr),* $(,)?)?
    ) => {
        #[doc = $docs]
        pub fn $fn(&self) -> String {
            self.$fn_ptr().to_string_lossy().into()
        }

        #[doc = "The lossless yet fallible version of [`"]
        #[doc = stringify!($fn)]
        #[doc = "]`."]
        pub fn $raw_fn(&self) -> Result<&str, $crate::Error> {
            self.$fn_ptr()
                .to_str()
                .map_err($crate::Error::ConvertCString)
        }

        fn $fn_ptr(&self) -> &std::ffi::CStr {
            unsafe {
                std::ffi::CStr::from_ptr($libui_fn(
                    $(
                        $($arg),*
                    )?
                    self.as_ptr(),
                ))
            }
        }
    };
}

macro_rules! bind_set_text_fn {
    (
        docs: $docs:literal,
        $fn:ident,
        $arg:ident,
        $libui_fn:ident $(,)?
    ) => {
        #[doc = $docs]
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
    (
        docs: $docs:literal,
        $fn:ident,
        $libui_fn:ident $(,)?
    ) => {
        #[doc = $docs]
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
    (
        docs: $docs:literal,
        $fn:ident,
        $libui_fn:ident $(,)?
    ) => {
        #[doc = $docs]
        pub fn $fn(&self, value: bool) {
            unsafe { $libui_fn(self.as_ptr(), value.into()) };
        }
    };
}
