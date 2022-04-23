// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use std::ffi::CString;

impl Ui {
    /// Creates a new [`Menu`].
    pub fn create_menu(&mut self, name: impl Into<Vec<u8>>) -> Result<Menu, crate::Error> {
        let name = CString::new(name).map_err(crate::Error::ConvertString)?;
        call_libui_new_fn!(Menu, Menu, name.as_ptr())
    }
}

def_subcontrol_with_ptr_ty!(Menu, uiMenu);

pub enum ItemKind {
    Generic { name: String },
    Check { name: String },
    Quit,
    Preferences,
    About,
}

impl Menu {
    pub fn append_item(&mut self, kind: ItemKind) -> Result<Item, crate::Error> {
        match kind {
            ItemKind::Generic { name } => self.append_generic_item(name),
            ItemKind::Check { name } => self.append_check_item(name),
            ItemKind::Quit => self.append_quit_item(),
            ItemKind::Preferences => self.append_preferences_item(),
            ItemKind::About => self.append_about_item(),
        }
    }

    fn append_generic_item(&mut self, _name: String) -> Result<Item, crate::Error> {
        todo!()
    }

    fn append_check_item(&mut self, _name: String) -> Result<Item, crate::Error> {
        todo!()
    }

    fn append_quit_item(&mut self) -> Result<Item, crate::Error> {
        todo!()
    }

    fn append_preferences_item(&mut self) -> Result<Item, crate::Error> {
        todo!()
    }

    fn append_about_item(&mut self) -> Result<Item, crate::Error> {
        todo!()
    }
}

def_subcontrol_with_ptr_ty!(Item, uiMenuItem);
