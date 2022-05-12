// SPDX-License-Identifier: MPL-2.0

use super::Menu;
use crate::prelude::*;

// Contrary to intuition, it is perfectly acceptable for the lifetime of a menu item to be different
// or even longer than that of its parent menu. This is because the [`Drop`] implementation of
// [`Menu`] is a stub, so drop order is irrelevant.
//
// However, the lifetime of a menu item *is* bound to the [`Ui`] object that created its menu. This
// is because, like menus, menu items cannot be destroyed, so *libui-ng* assumes they (and, by
// extension, their containing callback) live until the final invocation of `uiQuit`. For this
// reason, menu items are allocated in the [`Ui`] object's arena with [`Ui::alloc_object`].

macro_rules! impl_append_item_fn_with_name {
    ($boing_fn:ident, $libui_fn:ident) => {
        impl Menu {
            ///
            ///
            /// # Examples
            ///
            /// ```no_run
            /// // TODO
            /// ```
            pub fn $boing_fn<'ui>(&self, ui: &'ui Ui, name: impl AsRef<str>) -> Result<&'ui mut Item<'ui>, $crate::Error> {
                // `name` is dropped at the end of scope, at which point the underling string buffer
                // is freed, but that's OK! The `uiMenuItemAppend*Item` functions `strdup` the
                // string argument.
                let name = make_cstring!(name.as_ref());

                call_fallible_libui_fn!($libui_fn(self.as_ptr(), name.as_ptr()))
                    .map(|ptr| ui.alloc_object(Item::new(ptr)))
            }
        }
    };
}

macro_rules! impl_append_item_fn {
    ($boing_fn:ident, $libui_fn:ident) => {
        impl Menu {
            ///
            ///
            /// # Examples
            ///
            /// ```no_run
            /// // TODO
            /// ```
            pub fn $boing_fn<'ui>(&self, ui: &'ui Ui) -> Result<&'ui mut Item<'ui>, $crate::Error> {
                call_fallible_libui_fn!($libui_fn(self.as_ptr()))
                    .map(|ptr| ui.alloc_object(Item::new(ptr)))
            }
        }
    };
}

impl_append_item_fn_with_name!(append_item, uiMenuAppendItem);
impl_append_item_fn_with_name!(append_check_item, uiMenuAppendCheckItem);
impl_append_item_fn!(append_quit_item, uiMenuAppendQuitItem);
impl_append_item_fn!(append_preferences_item, uiMenuAppendPreferencesItem);
impl_append_item_fn!(append_about_item, uiMenuAppendAboutItem);

impl Item<'_> {
    pub(super) fn new(ptr: *mut uiMenuItem) -> Self {
        Self {
            ptr,
            on_clicked: None,
        }
    }
}

// Menu items are *not* controls as they are not backed by a `uiControl`. Do not use them as such!
// See the note above the definition of [`Menu`] in *menu.rs* for more information.

pub struct Item<'ui> {
    ptr: *mut uiMenuItem,
    on_clicked: Option<Box<dyn 'ui + FnMut(&mut Self)>>,
}

impl<'ui> Item<'ui> {
    pub(crate) fn as_ptr(&self) -> *mut uiMenuItem {
        self.ptr
    }

    bind_fn!(
        docs: "


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: enable() },
        libui: { fn: uiMenuItemEnable() },
    );

    bind_fn!(
        docs: "


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: disable() },
        libui: { fn: uiMenuItemDisable() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when this item is clicked.

            # Examples

            ```no_run
            // TODO
            ```
        ",
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
        docs: "
            Determines if this item is checked.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: is_checked() -> bool },
        libui: { fn: uiMenuItemChecked() },
    );

    bind_fn!(
        docs: "
            Sets whether or not this item is checked.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_checked(value: bool) },
        libui: { fn: uiMenuItemSetChecked() },
    );
}
