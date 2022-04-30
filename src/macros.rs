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
        pub struct $ty($crate::Control);

        impl $ty {
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
    ($fn:ident $(, $($arg:expr),* $(,)?)?) => {
        unsafe { $fn( $($($arg),*)? ).as_mut() }
            .ok_or($crate::Error::LibuiFn { name: stringify!($fn), cause: None })
    };
}

macro_rules! call_libui_new_fn {
    ($ui:expr, $ui_should_manage:expr, $out_ty:ident, $fn:ident $(, $($arg:expr),* $(,)?)?) => {
        call_fallible_libui_fn!($fn, $($($arg),*)?)
            .map(|control| unsafe {
                let control: *mut _ = control;
                let control = $ui.add_control(control.cast(), $ui_should_manage);

                $out_ty(control)
            })
    };
}

macro_rules! bind_callback_fn {
    (
        $docs:literal,
        $fn:ident,
        $libui_fn:ident
        $(, $($libui_arg:expr),* )? ;
        $user_cb:ident -> $user_cb_out:ty
        $( : $map_user_cb:expr )?,
        $libui_cb_out:ty,
        $self_handle:ident $(,)?
        $( : $cb_arg:ty ),*
    ) => {
        #[doc = $docs]
        #[allow(clippy::unused_unit)]
        pub fn $fn<F>(&mut self, $user_cb: F)
        where
            F: FnMut() -> $user_cb_out + 'static,
        {
            unsafe extern "C" fn callback<F>(
                _: *mut $self_handle,
                $(_: $cb_arg,)*
                data: *mut std::os::raw::c_void,
            ) -> $libui_cb_out
            where
                F: FnMut() -> $user_cb_out,
            {
                let $user_cb: &mut Box<F> = &mut *data.cast();
                let result = $user_cb();

                $(
                    let result = $map_user_cb(result);
                )?

                result
            }

            unsafe {
                $libui_fn(
                    self.as_ptr(),
                    $(
                        $($libui_arg),*
                    )?
                    Some(callback::<F>),
                    Box::into_raw(Box::new($user_cb)).cast(),
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
        pub fn $raw_fn(&self) -> Result<&str, crate::Error> {
            self.$fn_ptr()
                .to_str()
                .map_err(crate::Error::ConvertCString)
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
    ($docs:literal, $fn:ident, $arg:ident, $libui_fn:ident $(,)?) => {
        #[doc = $docs]
        pub fn $fn(&mut self, $arg: impl AsRef<str>) -> Result<(), crate::Error> {
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
        pub fn $fn(&mut self, value: bool) {
            unsafe { $libui_fn(self.as_ptr(), value.into()) };
        }
    };
}
