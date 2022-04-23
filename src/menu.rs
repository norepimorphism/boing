// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Menu`].
    pub fn create_menu(&mut self, name: impl Into<Vec<u8>>) -> Result<Menu, crate::Error> {
        call_libui_new_fn!(Menu, uiNewMenu, make_cstring!(name).as_ptr())
    }
}

def_subcontrol_with_ptr_ty!(Menu, uiMenu);

macro_rules! impl_append_item_fn_with_name {
    ($boing_fn:ident, $libui_fn:ident) => {
        impl Menu {
            pub fn $boing_fn(&mut self, name: impl Into<Vec<u8>>) -> Result<Item, $crate::Error> {
                call_libui_new_subitem_fn!(
                    self,
                    Item,
                    $libui_fn,
                    self.as_ptr(),
                    make_cstring!(name).as_ptr(),
                )
            }
        }
    };
}

macro_rules! impl_append_item_fn {
    ($boing_fn:ident, $libui_fn:ident) => {
        impl Menu {
            pub fn $boing_fn(&mut self) -> Result<Item, $crate::Error> {
                call_libui_new_subitem_fn!(self, Item, $libui_fn, self.as_ptr())
            }
        }
    };
}

impl_append_item_fn_with_name!(append_item, uiMenuAppendItem);
impl_append_item_fn_with_name!(append_check_item, uiMenuAppendCheckItem);
impl_append_item_fn!(append_quit_item, uiMenuAppendQuitItem);
impl_append_item_fn!(append_preferences_item, uiMenuAppendPreferencesItem);
impl_append_item_fn!(append_about_item, uiMenuAppendAboutItem);

def_subcontrol_subitem_with_ptr_ty!(Item, Menu, uiMenuItem);
