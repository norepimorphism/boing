// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

macro_rules! make_cstring {
    ($contents:expr) => {
        std::ffi::CString::new($contents).map_err($crate::Error::ConvertString)?
    };
}

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

macro_rules! def_subcontrol_subitem {
    ($ty:ident, $parent_ty:ident $(,)?) => {
        pub struct $ty<'a> {
            control: Control,
            _parent: &'a $parent_ty,
        }

        impl std::ops::Deref for $ty<'_> {
            type Target = Control;

            fn deref(&self) -> &Self::Target {
                &self.control
            }
        }

        impl std::ops::DerefMut for $ty<'_> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.control
            }
        }
    };
}

macro_rules! def_subcontrol_subitem_with_ptr_ty {
    ($ty:ident, $parent_ty:ident, $ptr_ty:ty $(,)?) => {
        def_subcontrol_subitem!($ty, $parent_ty);

        impl<'a> $ty<'a> {
            pub unsafe fn from_ptr(parent: &'a $parent_ty, ptr: *mut $ptr_ty) -> Self {
                Self {
                    control: $crate::Control::from_ptr(ptr.cast()),
                    _parent: parent,
                }
            }

            pub fn as_ptr(&self) -> *mut $ptr_ty {
                self.control.as_ptr().cast()
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
    ($out_ty:ty, $fn:ident $(, $($arg:expr),* $(,)?)?) => {
        call_fallible_libui_fn!($fn, $($($arg),*)?)
            .map(|it| unsafe { <$out_ty>::from_ptr(it) })
    };
}

macro_rules! call_libui_new_subitem_fn {
    ($parent:expr, $out_ty:ty, $fn:ident $(, $($arg:expr),* $(,)?)?) => {
        call_fallible_libui_fn!($fn, $($($arg),*)?)
            .map(|it| unsafe { <$out_ty>::from_ptr($parent, it) })
    };
}
