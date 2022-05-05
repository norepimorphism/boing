// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

macro_rules! make_cstring {
    ($contents:expr $(,)?) => {
        std::ffi::CString::new($contents).map_err($crate::Error::ConvertRustString)?
    };
}

macro_rules! def_subcontrol {
    ($ty:ident, $ptr_ty:ident $(,)?) => {
        #[derive(Debug, Eq, PartialEq)]
        pub struct $ty($crate::Control);

        impl $ty {
            unsafe fn from_ptr(ptr: *mut $ptr_ty) -> Self {
                Self::from_control(Control::from_ptr(ptr.cast()))
            }

            pub(crate) unsafe fn from_control(control: Control) -> Self {
                Self(control)
            }

            pub fn as_ptr(&self) -> *mut $ptr_ty {
                self.0.as_ptr().cast()
            }
        }

        impl std::ops::Deref for $ty {
            type Target = Control;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

macro_rules! call_fallible_libui_fn {
    ($fn:ident $(, $($arg:expr),* $(,)? )?) => {
        unsafe { $fn( $($($arg),*)? ).as_mut() }
            .ok_or($crate::Error::LibuiFn { name: stringify!($fn), cause: None })
    };
}

macro_rules! call_libui_new_fn {
    ($ui:expr, $out_ty:ident, $fn:ident $(, $($arg:expr),* $(,)? )?) => {
        call_fallible_libui_fn!($fn, $($($arg),*)?)
            .map(|ptr| unsafe {
                $ui.alloc($out_ty::from_ptr(ptr))
            })
    };
}

macro_rules! bind_callback_fn {
    (
        $docs:literal,
        $self_ty:ty,
        $fn:ident,
        $libui_fn:ident
        $(, $($libui_arg:expr),* )? ;
        $user_cb:ident -> $user_cb_out:ty
        $(, : $map_user_cb:expr )?,
        $libui_cb_out:ty,
        : $self_handle_ty:ident
        $(,  : $cb_arg:ty ),* $(,)?
    ) => {
        #[doc = $docs]
        pub fn $fn<F>(&self, ui: &Ui, $user_cb: F)
        where
            F: 'static + FnMut(&Ui, &mut $self_ty) -> $user_cb_out,
        {
            unsafe extern "C" fn trampoline<F>(
                handle: *mut libui_ng_sys::$self_handle_ty,
                $(_: $cb_arg,)*
                data: *mut std::os::raw::c_void,
            ) -> $libui_cb_out
            where
                F: FnMut(&Ui, &mut $self_ty) -> $user_cb_out,
            {
                debug_assert!(!data.is_null());
                let data: &mut Data<F> = &mut *data.cast();

                let mut handle = std::mem::ManuallyDrop::new(<$self_ty>::from_ptr(handle));
                let result = (data.user_cb)(data.ui, &mut handle);
                $(
                    let result = $map_user_cb(result);
                )?

                result
            }

            struct Data<'ui, F> {
                ui: &'ui Ui,
                user_cb: &'ui mut F,
            }

            let data = Data {
                ui,
                user_cb: ui.alloc($user_cb),
            };

            unsafe {
                $libui_fn(
                    self.as_ptr(),
                    $(
                        $($libui_arg),*
                    )?
                    Some(trampoline::<F>),
                    std::ptr::addr_of_mut!(*ui.alloc(data)).cast(),
                );
            }
        }
    };
}

macro_rules! bind_text_fn {
    (
        $docs:literal,
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

macro_rules! bind_add_child_fn {
    ($docs:literal, $fn:ident, $child:ident, $libui_fn:ident $(,)?) => {
        #[doc = $docs]
        pub fn $fn(&self, $child: &mut impl std::ops::DerefMut<Target = Control>) {
            let $child = std::mem::ManuallyDrop::new($child);
            unsafe { $libui_fn(self.as_ptr(), $child.as_ptr()) };
        }
    };
}

macro_rules! bind_set_text_fn {
    ($docs:literal, $fn:ident, $arg:ident, $libui_fn:ident $(,)?) => {
        #[doc = $docs]
        pub fn $fn(&self, $arg: impl AsRef<str>) -> Result<(), $crate::Error> {
            let $arg = make_cstring!($arg.as_ref());
            unsafe { $libui_fn(self.as_ptr(), $arg.as_ptr()) };

            Ok(())
        }
    };
}

macro_rules! bind_bool_fn {
    ($docs:literal, $fn:ident, $libui_fn:ident $(,)?) => {
        #[doc = $docs]
        pub fn $fn(&self) -> bool {
            unsafe { $libui_fn(self.as_ptr()) == 1 }
        }
    };
}

macro_rules! bind_set_bool_fn {
    ($docs:literal, $fn:ident, $libui_fn:ident $(,)?) => {
        #[doc = $docs]
        pub fn $fn(&self, value: bool) {
            unsafe { $libui_fn(self.as_ptr(), value.into()) };
        }
    };
}
