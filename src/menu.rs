// SPDX-License-Identifier: MPL-2.0

//! A drop-down menu within a menubar that may contain additional [menu items](`Item`).

pub mod item;

pub use item::Item;

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Menu`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_menu(&self, name: impl AsRef<str>) -> Result<Menu, crate::Error> {
        let name = make_cstring!(name.as_ref());
        call_fallible_libui_fn!(uiNewMenu(name.as_ptr())).map(|menu| Menu { ptr: menu })
    }
}

// Why is it that all widgets are controls except for [`Menu`] and [`Item`], you ask? It's simply
// because `uiMenu` is *not* backed by a `uiControl`. Whereas `uiWindow`, `uiButton`, and `uiLabel`
// are `uiControls` with additional metadata, `uiMenu` and `uiMenuItem` do not begin with a
// `uiControl`, and as such, it is unsound to cast them to `*mut uiControl` and use them so.

/// A drop-down menu within a menubar that may contain additional [menu items](`Item`).
pub struct Menu {
    ptr: *mut uiMenu,
}

impl Menu {
    bind_fn!(
        docs: "
            Inserts a separator below the last [item](Item).

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: append_separator() },
        libui: { fn: uiMenuAppendSeparator() },
    );

    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn as_ptr(&self) -> *mut uiMenu {
        self.ptr
    }
}
