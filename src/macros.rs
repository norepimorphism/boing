// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

macro_rules! make_cstring {
    ($contents:expr) => {
        std::ffi::CString::new($contents).map_err($crate::Error::ConvertString)?
    };
}

macro_rules! def_subcontrol {
    ($ty:ident, $ptr_ty:ident) => {
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

macro_rules! call_libui_new_fn {
    ($ui:expr, $out_ty:ident, $fn:ident $(, $($arg:expr),* $(,)?)?) => {
        unsafe { $fn( $($($arg),*)? ).as_mut() }
            .ok_or($crate::Error::LibuiFn { name: stringify!($fn), cause: None })
            .map(|control| unsafe {
                let control: *mut _ = control;
                $out_ty($ui.add_control(control.cast()))
            })
    };
}
