// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use super::Menu;

macro_rules! impl_append_item_fn_with_name {
    ($boing_fn:ident, $libui_fn:ident) => {
        impl Menu {
            pub fn $boing_fn(
                &mut self,
                name: impl Into<Vec<u8>>,
            ) -> Result<Item, $crate::Error> {
                let name = make_cstring!(name);
                call_fallible_libui_fn!(
                    $libui_fn,
                    self.as_ptr(),
                    name.as_ptr(),
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
        Self(ptr)
    }

    fn as_ptr(&self) -> *mut uiMenuItem {
        self.0
    }

    bind_callback_fn!(
        on_clicked,
        uiMenuItemOnClicked;
        (),
        uiMenuItem,
        : *mut uiWindow
    );

    bind_bool_fn!(
        is_checked,
        uiMenuItemChecked,
    );

    bind_set_bool_fn!(
        set_checked,
        uiMenuItemSetChecked,
    );
}
