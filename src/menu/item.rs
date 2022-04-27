use crate::prelude::*;
use super::Menu;

macro_rules! impl_append_item_fn_with_name {
    ($boing_fn:ident, $libui_fn:ident) => {
        impl Menu {
            pub fn $boing_fn(
                &mut self,
                name: impl Into<Vec<u8>>,
            ) -> Result<Item, $crate::Error> {
                call_fallible_libui_fn!(
                    $libui_fn,
                    self.as_ptr(),
                    make_cstring!(name).as_ptr(),
                )
                .map(|ptr| Item::from_ptr(ptr))
            }
        }
    };
}

macro_rules! impl_append_item_fn {
    ($boing_fn:ident, $libui_fn:ident) => {
        impl Menu {
            pub fn $boing_fn(&mut self) -> Result<Item, $crate::Error> {
                call_fallible_libui_fn!($libui_fn, self.as_ptr())
                    .map(|ptr| Item::from_ptr(ptr))
            }
        }
    };
}

impl_append_item_fn_with_name!(append_item, uiMenuAppendItem);
impl_append_item_fn_with_name!(append_check_item, uiMenuAppendCheckItem);
impl_append_item_fn!(append_quit_item, uiMenuAppendQuitItem);
impl_append_item_fn!(append_preferences_item, uiMenuAppendPreferencesItem);
impl_append_item_fn!(append_about_item, uiMenuAppendAboutItem);

pub struct Item(*mut uiMenuItem);

impl Item {
    pub(super) fn from_ptr(ptr: *mut uiMenuItem) -> Self {
        let item = Self(ptr);

        // #[cfg(target_os = "windows")]
        // item.apply_rounded_corners();

        item
    }

    /*
    #[cfg(target_os = "windows")]
    fn apply_rounded_corners(&self) {
        use std::{mem, ptr};
        use winapi::{um::dwmapi, shared::minwindef::DWORD};

        // <https://docs.microsoft.com/en-us/windows/win32/api/dwmapi/ne-dwmapi-dwmwindowattribute>
        const DWMWA_WINDOW_CORNER_PREFERENCE: DWORD = 33;
        // <https://docs.microsoft.com/en-us/windows/win32/api/dwmapi/ne-dwmapi-dwm_window_corner_preference>
        const DWMWCP_ROUND: DWORD = 2;

        let pref = DWMWCP_ROUND;
        unsafe {
            dwmapi::DwmSetWindowAttribute(
                handle.cast(),
                DWMWA_WINDOW_CORNER_PREFERENCE,
                ptr::addr_of!(pref).cast(),
                mem::size_of_val(&pref) as u32,
            );
        }
    }
    */

    fn as_ptr(&self) -> *mut uiMenuItem {
        self.0
    }
}
