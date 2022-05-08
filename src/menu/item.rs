// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use super::Menu;

macro_rules! impl_append_item_fn_with_name {
    ($boing_fn:ident, $libui_fn:ident) => {
        impl<'ui> Menu<'ui> {
            pub fn $boing_fn<'a>(
                &self,
                ui: &'a Ui<'ui>,
                name: impl AsRef<str>,
            ) -> Result<&'a mut Item<'ui>, $crate::Error> {
                let name = make_cstring!(name.as_ref());
                call_fallible_libui_fn!($libui_fn(self.as_ptr(), name.as_ptr()))
                    .map(|ptr| ui.alloc_menu_item(Item::from_ptr(ptr)))
            }
        }
    };
}

macro_rules! impl_append_item_fn {
    ($boing_fn:ident, $libui_fn:ident) => {
        impl<'ui> Menu<'ui> {
            pub fn $boing_fn<'a>(&self, ui: &'a Ui<'ui>) -> Result<&'a mut Item<'ui>, $crate::Error> {
                call_fallible_libui_fn!($libui_fn(self.as_ptr()))
                    .map(|ptr| ui.alloc_menu_item(Item::from_ptr(ptr)))
            }
        }
    };
}

impl_append_item_fn_with_name!(append_item, uiMenuAppendItem);
impl_append_item_fn_with_name!(append_check_item, uiMenuAppendCheckItem);
impl_append_item_fn!(append_quit_item, uiMenuAppendQuitItem);
impl_append_item_fn!(append_preferences_item, uiMenuAppendPreferencesItem);
impl_append_item_fn!(append_about_item, uiMenuAppendAboutItem);

pub struct Item<'ui> {
    ptr: *mut uiMenuItem,
    on_clicked: Option<*mut (dyn 'ui + FnMut())>,
}

impl<'ui> Item<'ui> {
    pub(super) fn from_ptr(ptr: *mut uiMenuItem) -> Self {
        Self {
            ptr,
            on_clicked: None,
        }
    }

    fn as_ptr(&self) -> *mut uiMenuItem {
        self.ptr
    }

    bind_callback_fn!(
        docs: "Sets a callback for when this item is clicked.",
        self: {
            ty: Item<'ui>,
            handle: uiMenuItem,
            fn: on_clicked(),
            cb: {
                sig: f -> (),
            },
        },
        libui: {
            fn: uiMenuItemOnClicked(),
            cb: {
                sig: (*mut uiWindow) -> (),
            },
        },
    );

    bind_bool_fn!(
        docs: "Determines if this item is checked.",
        is_checked,
        uiMenuItemChecked,
    );

    bind_set_bool_fn!(
        docs: "Sets whether or not this item is checked.",
        set_checked,
        uiMenuItemSetChecked,
    );
}
