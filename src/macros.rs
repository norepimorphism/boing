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
        $name:ident,
        $fn:expr
        $(, $($arg:expr),* )? ;
        $out:ty,
        $handle:ident $(,)?
        $( : $cb_ty:ty ),*
    ) => {
        #[allow(clippy::unused_unit)]
        pub fn $name<F, A>(&mut self, f: F, arg: A)
        where
            F: FnMut(&mut A) -> $out + 'static,
        {
            struct Data<F, A> {
                f: F,
                arg: A,
            }

            unsafe extern "C" fn call_closure<F, A>(
                _: *mut $handle,
                $(_: $cb_ty,)*
                data: *mut std::os::raw::c_void,
            ) -> $out
            where
                F: FnMut(&mut A) -> $out,
            {
                let data: &mut Data<F, A> = &mut *data.cast();
                (data.f)(&mut data.arg)
            }

            unsafe {
                $fn(
                    self.as_ptr(),
                    $(
                        $($arg),*
                    )?
                    Some(call_closure::<F, A>),
                    Box::into_raw(Box::new(Data { f, arg })).cast(),
                );
            }
        }
    };
}

macro_rules! bind_text_fn {
    ($name:ident, $raw_name:ident, $name_ptr:ident, $libui_fn:expr $(, $($arg:expr),* $(,)?)?) => {
        pub fn $name(&self) -> String {
            self.$name_ptr().to_string_lossy().into()
        }

        pub fn $raw_name(&self) -> Result<&str, crate::Error> {
            self.$name_ptr()
                .to_str()
                .map_err(crate::Error::ConvertCString)
        }

        fn $name_ptr(&self) -> &std::ffi::CStr {
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
    ($name:ident, $arg:ident, $libui_fn:ident $(,)?) => {
        pub fn $name(&mut self, $arg: impl AsRef<str>) -> Result<(), crate::Error> {
            let $arg = make_cstring!($arg.as_ref());
            unsafe { $libui_fn(self.as_ptr(), $arg.as_ptr()) };

            Ok(())
        }
    };
}

macro_rules! bind_bool_fn {
    ($name:ident, $fn:ident $(,)?) => {
        pub fn $name(&self) -> bool {
            unsafe { $fn(self.as_ptr()) == 1 }
        }
    };
}

macro_rules! bind_set_bool_fn {
    ($name:ident, $fn:ident $(,)?) => {
        pub fn $name(&mut self, value: bool) {
            unsafe { $fn(self.as_ptr(), value.into()) };
        }
    };
}
