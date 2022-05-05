// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use super::Menu;

macro_rules! impl_append_item_fn_with_name {
    ($boing_fn:ident, $libui_fn:ident) => {
        impl Menu {
            pub fn $boing_fn<'a>(
                &self,
                ui: &'a Ui,
                name: impl AsRef<str>,
            ) -> Result<&'a mut Item, $crate::Error> {
                let name = make_cstring!(name.as_ref());
                call_fallible_libui_fn!(
                    $libui_fn,
                    self.as_ptr(),
                    name.as_ptr(),
                )
                .map(|ptr| ui.alloc(Item::from_ptr(ptr)))
            }
        }
    };
}

macro_rules! impl_append_item_fn {
    ($boing_fn:ident, $libui_fn:ident) => {
        impl Menu {
            pub fn $boing_fn<'a>(&self, ui: &'a Ui) -> Result<&'a mut Item, $crate::Error> {
                call_fallible_libui_fn!($libui_fn, self.as_ptr())
                    .map(|ptr| ui.alloc(Item::from_ptr(ptr)))
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
        "Sets a callback for when this item is clicked.",
        Item,
        on_clicked,
        uiMenuItemOnClicked;
        f -> (),
        (),
        : uiMenuItem,
        : *mut uiWindow,
    );

    bind_bool_fn!(
        "Determines if this item is checked.",
        is_checked,
        uiMenuItemChecked,
    );

    bind_set_bool_fn!(
        "Sets whether or not this item is checked.",
        set_checked,
        uiMenuItemSetChecked,
    );
}
