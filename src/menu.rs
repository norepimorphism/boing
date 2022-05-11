// SPDX-License-Identifier: MPL-2.0

//! A drop-down menu within a menubar that may contain additional [menu items](`Item`).

pub mod item;

pub use item::Item;

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Menu`].
    pub fn create_menu(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Menu, crate::Error> {
        let name = make_cstring!(name.as_ref());
        call_fallible_libui_fn!(uiNewMenu(name.as_ptr())).map(|menu| {
            Menu {
                ptr: menu,
            }
        })
    }
}

/// A drop-down menu within a menubar that may contain additional [menu items](`Item`).
pub struct Menu {
    ptr: *mut uiMenu,
}

impl Menu {
    pub fn as_ptr(&self) -> *mut uiMenu {
        self.ptr
    }

    pub fn append_separator(&self) {
        unsafe { uiMenuAppendSeparator(self.as_ptr()) };
    }
}
