// SPDX-License-Identifier: MPL-2.0

/// Converts an `i32` to a `u16`, panicking and providing a helpful error message if the conversion
/// fails.
macro_rules! to_u16 {
    ($i32:expr) => {
        u16::try_from($i32).expect(indoc::indoc! {"
            Failed to convert an `i32` to `u16`.

            *libui-ng* uses the C type `int` for nearly all numeric values, including ones that are
            semantically unsigned. For saneness, *boing* automatically attempts to convert integers
            to an unsigned representation in such cases, exposing a `u16`-based interface to the end
            user.

            Unfortunately, it appears this conversion failed, and *libui-ng* returned a negative
            integer where a positive integer was expected. Please file an issue to the *libui-ng*
            GitHub repository. Thanks!
        "})
    };
}

/// Creates a new `CString`, returning from the current scope if an error occurs during the
/// conversion.
///
/// This macro is commonly used such that the returned `CString` is immediately dropped after a
/// pointer to its buffer is passed to a *libui-ng* function. This seems like a typical
/// use-after-free, but it's actually OK! As far as I can tell, *libui-ng* (or the underlying OS UI
/// libraries) `strdup` string arguments.
macro_rules! make_cstring {
    ($contents:expr $(,)?) => {
        std::ffi::CString::new($contents).map_err($crate::Error::ConvertRustString)?
    };
}

/// Defines a new control type.
///
/// Note that this macro should not and cannot be used on all widgets, and in particular,
/// [`crate::Menu`] and [`crate::MenuItem`].
macro_rules! def_subcontrol {
    (
        docs: $docs:literal,
        ty: $ty:ident,
        handle: $ptr_ty:ident
        $(
            , cb_fns: [
                $($cb:ident<$lt:lifetime>() $(-> $out:ty)?),* $(,)?
            ]
        )?
        $(
            , fields: [
                $($field_name:ident : $field_ty:ty = $field_default:expr),* $(,)?
            ]
        )? $(,)?
    ) => {
        impl$(<$($lt),*>)? $ty$(<$($lt),*>)? {
            pub(crate) fn new(ptr: *mut $ptr_ty) -> Self {
                // Here, we cast `ptr` to `*mut uiControl`. This is safe because the memory layout
                // of all subcontrols begins with a `uiControl` struct, so they are effectively
                // subclasses of `uiControl`.
                Self::from_control(Control::new(ptr.cast()))
            }

            pub(crate) fn from_control(control: Control) -> Self {
                Self {
                    inner: control
                    $(
                        , $($cb: None),*
                    )?
                    $(
                        , $($field_name: $field_default),*
                    )?
                }
            }
        }

        #[doc = indoc::indoc!($docs)]
        pub struct $ty$(<$($lt),*>)? {
            inner: $crate::Control
            $(
                , $($cb: Option<Box<dyn $lt + FnMut(&mut Self) $(-> $out)?>>),*
            )?
            $(
                , $($field_name: $field_ty),*
            )?
        }

        impl$(<$($lt),*>)? $ty$(<$($lt),*>)? {
            /// A handle to the underlying *libui-ng* object.
            ///
            /// # Examples
            ///
            /// ```no_run
            /// // TODO
            /// ```
            pub fn as_ptr(&self) -> *mut $ptr_ty {
                // This is OK for the same reason outlined in [`Self::new`].
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
        // `<pointer>::as_mut` checks for NULL.
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
            .map(|ptr| $ui.alloc_object($out_ty::new(ptr)))
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
                sig: ($($cb_arg:ty),*) -> $libui_cb_out:ty $(,)?
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
                user_cb_ptr: *mut std::os::raw::c_void,
            ) -> $libui_cb_out {
                // Ensure nothing wonky has happened in the meantime.
                debug_assert!(!handle.is_null());
                debug_assert!(!user_cb_ptr.is_null());

                let user_cb: &mut Option<Box<dyn FnMut(&mut $self_ty) -> $user_cb_out>> = &mut *user_cb_ptr.cast();

                // SAFETY:
                //
                // The [`Option` docs] state that `Some(T)` is equivalent in memory to `T`. In this
                // case, the `Option` is definitely `Some` (unless something wonky happened!), so
                // this should be safe.
                //
                // [`Option` docs]: https://doc.rust-lang.org/std/option/index.html#representation
                debug_assert!(user_cb.is_some());
                let user_cb: &mut Box<dyn FnMut(&mut $self_ty) -> $user_cb_out> = &mut *user_cb_ptr.cast();

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
            fn: $self_fn:ident<$ui_lt:lifetime>($self_child:ident $(,)?) $(,)?
        },
        libui: {
            fn: $libui_fn:ident() $(,)?
        } $(,)?
    ) => {
        #[doc = indoc::indoc!($docs)]
        pub fn $self_fn(&self, $self_child: &impl std::ops::DerefMut<Target = Control>) {
            // Inform the child control that it should not destroy itself as *libui-ng* will take
            // care of that for us.
            $self_child.make_child();

            unsafe { $libui_fn(self.as_ptr(), $self_child.as_ptr()) };
        }
    };
}

/// Defines a binding to a *libui-ng-sys* function that returns a C-style string.
macro_rules! bind_text_fn {
    (
        docs: $docs:literal,
        self: {
            fn: $self_fn:ident(),
            raw_fn: $self_raw_fn:ident(),
            as_ptr_fn: $self_as_ptr_fn:ident() $(,)?
        },
        libui: {
            fn: $libui_fn:ident($($libui_arg:expr),* $(,)?) $(,)?
        } $(,)?
    ) => {
        #[doc = indoc::indoc!($docs)]
        pub fn $self_fn(&self) -> String {
            let ptr = self.$self_as_ptr_fn();
            let string = unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into() };

            // Now that the contents of `ptr` have been copied into `string`, we can safely free the
            // original string.
            unsafe { uiFreeText(ptr) };

            string
        }

        #[doc = concat!(
            "The lossless yet fallible version of [`Self::", stringify!($self_fn), "`]."
        )]
        #[doc = indoc::indoc!("
            # Examples

            ```no_run
            // TODO
            ```
        ")]
        pub fn $self_raw_fn(&self) -> Result<&str, $crate::Error> {
            unsafe { std::ffi::CStr::from_ptr(self.$self_as_ptr_fn()) }
                .to_str()
                .map_err($crate::Error::ConvertCString)
        }

        ///
        ///
        /// # Examples
        ///
        /// ```no_run
        /// // TODO
        /// ```
        pub fn $self_as_ptr_fn(&self) -> *mut std::os::raw::c_char {
            unsafe {
                $libui_fn(
                    $(
                        $($libui_arg),*
                    )?
                    self.as_ptr(),
                )
            }
        }
    };
}

/// Defines a binding to a *libui-ng-sys* function that accepts a C-style string.
macro_rules! bind_set_text_fn {
    (
        docs: $docs:literal,
        self $( ( $mut_spec:tt ) )? : {
            fn: $self_fn:ident(
                $self_arg:ident
                $(, $(^ $self_pre_arg:ident : $self_pre_ty:ty $(=> $self_pre_map:expr)?),*)? $(,)?
            ) -> $self_fn_out:ty
            $(, map_out: $self_map_out:expr)? $(,)?
        },
        libui: {
            fn: $libui_fn:ident() $(,)?
        } $(,)?
    ) => {
        #[doc = indoc::indoc!($docs)]
        pub fn $self_fn(
            & $($mut_spec)? self,
            $($($self_pre_arg: $self_pre_ty,)*)?
            $self_arg: impl AsRef<str>,
        ) -> Result<$self_fn_out, $crate::Error> {
            $(
                $(
                    $(
                        let $self_pre_arg = $self_pre_map($self_pre_arg);
                    )?
                )*
            )?

            // Normally, this is a bad idea: `$arg` is a `CString` that will be dropped at the end
            // of this scope, but its pointer is being passed to a C function that may, in theory,
            // refer to it indefinitiely long. However, as far as I know, *libui-ng* `strdup`s all
            // string arguments before using them, so this should be safe.
            let $self_arg = make_cstring!($self_arg.as_ref());

            let result = unsafe {
                $libui_fn(self.as_ptr(), $($($self_pre_arg.into(),)*)? $self_arg.as_ptr())
            };

            $(
                let result = $self_map_out(self, result);
            )?

            Ok(result)
        }
    };
}

/// Defines a binding to a *libui-ng-sys* function that returns `bool`.
macro_rules! bind_bool_fn {
    (
        docs: $docs:literal,
        self: {
            fn: $self_fn:ident(
                $($self_arg:ident : $self_ty:ty $(=> $self_arg_map:expr)? ),* $(,)?
            ) -> bool $(,)?
        },
        libui: {
            fn: $libui_fn:ident() $(,)?
        } $(,)?
    ) => {
        #[doc = indoc::indoc!($docs)]
        pub fn $self_fn(&self, $($self_arg: $self_ty),*) -> bool {
            $(
                $(
                    let $self_arg = $self_arg_map($self_arg);
                )?
            )*

            let result = unsafe { $libui_fn(self.as_ptr(), $($self_arg.into()),*) };

            // A boolean should be 0 or 1; if not, something may be awry.
            debug_assert!((result == 0) || (result == 1));

            // We could test `result == 1`, but many C programs test booleans with `if(_)`---which
            // is shorthand for `if(_ != 0)`---so we will replicate that functionality here.
            result != 0
        }
    };
}

/// Defines a binding to a *libui-ng-sys* function.
macro_rules! bind_fn {
    (
        docs: $docs:literal,
        self $( ( $mut_spec:tt ) )?  : {
            fn: $self_fn:ident(
                $($self_arg:ident : $self_ty:ty $(=> $self_arg_map:expr)?),* $(,)?
            ) $(-> $self_fn_out:ty)?
            $(, map_out: $self_map_out:expr)? $(,)?
        },
        libui: {
            fn: $libui_fn:ident($($libui_arg:expr),* $(,)?) $(,)?
        } $(,)?
    ) => {
        #[doc = indoc::indoc!($docs)]
        pub fn $self_fn(& $($mut_spec)? self, $($self_arg: $self_ty),*) $(-> $self_fn_out)? {
            $(
                $(
                    let $self_arg = $self_arg_map($self_arg);
                )?
            )*

            let result = unsafe { $libui_fn(self.as_ptr(), $($self_arg.into()),* $($libui_arg),*) };
            $(
                let result = $self_map_out(self, result);
            )?

            result
        }
    };
}
