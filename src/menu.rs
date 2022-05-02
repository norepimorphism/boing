// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Menu`] and [`Item`].

pub mod item;

pub use item::Item;

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Menu`].
    pub fn create_menu(&self, name: impl AsRef<str>) -> Result<&mut Menu, crate::Error> {
        let name = make_cstring!(name.as_ref());
        call_fallible_libui_fn!(
            uiNewMenu,
            name.as_ptr(),
        )
        .map(|menu| {
            self.alloc_object(Menu(menu))
        })
    }
}

pub struct Menu(*mut uiMenu);

impl Menu {
    pub fn as_ptr(&self) -> *mut uiMenu {
        self.0
    }

    pub fn append_separator(&mut self) {
        unsafe { uiMenuAppendSeparator(self.as_ptr()) };
    }
}
