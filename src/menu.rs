// SPDX-License-Identifier: MPL-2.0

//! A drop-down menu within a menubar that may contain additional [menu items](`Item`).

mod item;

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
        call_fallible_libui_fn!(uiNewMenu(name.as_ptr())).map(|menu| Menu { ptr: menu, ui: self })
    }
}

// Why is it that all widgets are controls except for [`Menu`] and [`Item`], you ask? It's simply
// because `uiMenu` is *not* backed by a `uiControl`. Whereas `uiWindow`, `uiButton`, and `uiLabel`
// are `uiControls` with additional metadata, `uiMenu` and `uiMenuItem` do not begin with a
// `uiControl` in memory, and as such, it is unsound to cast them to `*mut uiControl` and use them
// so.

/// An application-wide drop-down menu within a menubar that may contain additional
/// [menu items](`Item`).
pub struct Menu<'ui> {
    ptr: *mut uiMenu,
    ui: &'ui Ui,
}

impl Menu<'_> {
    bind_fn!(
        docs: "
            Appends a horizontal separator.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: push_separator() },
        libui: { fn: uiMenuAppendSeparator() },
    );

    /// A handle to the underlying *libui-ng* menu object.
    ///
    /// # Safety
    ///
    /// The returned pointer is guaranteed to be non-null. Beyond that, it is your responsibility to
    /// use the handle appropriately. Consulting the *libui-ng* documentation or source code will be
    /// of utility in this regard, as well as the *boing* source code. See *[libui-ng-sys]* for
    /// *libui-ng* bindings.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    ///
    /// [libui-ng-sys]: https://github.com/norepimorphism/libui-ng-sys
    pub fn as_ptr(&self) -> *mut uiMenu {
        self.ptr
    }
}
