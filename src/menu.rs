// SPDX-License-Identifier: MPL-2.0

//! [`Menu`] and [`Item`].

pub mod item;

use std::marker::PhantomData;

pub use item::Item;

use crate::prelude::*;

impl<'ui> Ui<'ui> {
    /// Creates a new [`Menu`].
    pub fn create_menu<'a>(
        &'a self,
        name: impl AsRef<str>,
    ) -> Result<&'a mut Menu<'ui>, crate::Error> {
        let name = make_cstring!(name.as_ref());
        call_fallible_libui_fn!(uiNewMenu(name.as_ptr())).map(|menu| {
            self.alloc_menu(Menu::<'ui> {
                ptr: menu,
                _ui: PhantomData,
            })
        })
    }
}

pub struct Menu<'ui> {
    ptr: *mut uiMenu,
    _ui: PhantomData<&'ui Ui<'ui>>,
}

impl Menu<'_> {
    pub fn as_ptr(&self) -> *mut uiMenu {
        self.ptr
    }

    pub fn append_separator(&self) {
        unsafe { uiMenuAppendSeparator(self.as_ptr()) };
    }
}
