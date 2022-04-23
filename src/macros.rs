// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

macro_rules! def_subcontrol {
    ($ty:ident) => {
        pub struct $ty($crate::Control);

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

macro_rules! def_subcontrol_with_ptr_ty {
    ($ty:ident, $ptr_ty:ty $(,)?) => {
        def_subcontrol!($ty);

        impl $ty {
            pub unsafe fn from_ptr(ptr: *mut $ptr_ty) -> Self {
                Self($crate::Control::from_ptr(ptr.cast()))
            }

            pub fn as_ptr(&self) -> *mut $ptr_ty {
                self.0.as_ptr().cast()
            }
        }
    };
}

macro_rules! call_fallible_libui_fn {
    ($name:ident, $($arg:expr),* $(,)?) => {
        unsafe { $name( $($arg),* ).as_mut() }
            .ok_or($crate::Error::LibuiFn { name: stringify!($name), cause: None })
    };
}

macro_rules! call_libui_new_fn {
    ($boing_ty:ty, $libui_ty:ident, $($arg:expr),* $(,)?) => {
        {
            let fn_name = concat_idents!(uiNew, $libui_ty);
            call_fallible_libui_fn!(fn_name, $($arg),*)
                .map(|it| unsafe { <$boing_ty>::from_ptr(it) })
        }
    };
}
