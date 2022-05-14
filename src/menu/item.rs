// SPDX-License-Identifier: MPL-2.0

//! An item within a [`Menu`].

use super::Menu;
use crate::prelude::*;

// Contrary to intuition, it is perfectly acceptable for the lifetime of a menu item to be different
// or even longer than that of its parent menu. This is because the [`Drop`] implementation of
// [`Menu`] is a stub, so drop order is irrelevant.
//
// However, the lifetime of a menu item *is* bound to the [`Ui`] object that created its menu, as
// menu items contain callbacks that are executed in [`Ui::run`]. For this reason, like normal
// controls, menu items are allocated in the [`Ui`] object's arena with [`Ui::alloc_object`].

macro_rules! impl_push_item_fn_with_name {
    (
        $docs:literal,
        $boing_fn:ident,
        $libui_fn:ident $(,)?
    ) => {
        impl<'ui> Menu<'ui> {
            #[doc = indoc::indoc!($docs)]
            pub fn $boing_fn(
                &self,
                text: impl AsRef<str>,
            ) -> Result<&'ui mut Item<'ui>, $crate::Error> {
                // SAFETY:
                //
                // `text` is dropped at the end of scope, at which point the underling string buffer
                // is freed, but that's OK! The `uiMenuItemAppend*Item` functions `strdup` the
                // string argument.
                let text = make_cstring!(text.as_ref());

                call_fallible_libui_fn!($libui_fn(self.as_ptr(), text.as_ptr()))
                    .map(|ptr| {
                        // SAFETY: `call_fallible_libui_fn` guarantees that the mapped pointer is
                        // non-null.
                        let item = unsafe { Item::new(ptr) };

                        // SAFETY: Items own callbacks, so they must live for the duration of
                        // `self.ui`.
                        self.ui.alloc_object(item)
                    })
            }
        }
    };
}

macro_rules! impl_push_item_fn {
    (
        $docs:literal,
        $boing_fn:ident,
        $libui_fn:ident $(,)?
    ) => {
        impl<'ui> Menu<'ui> {
            #[doc = indoc::indoc!($docs)]
            pub fn $boing_fn(&self) -> Result<&'ui mut Item<'ui>, $crate::Error> {
                call_fallible_libui_fn!($libui_fn(self.as_ptr()))
                    .map(|ptr| {
                        // SAFETY: `call_fallible_libui_fn` guarantees that the mapped pointer is
                        // non-null.
                        let item = unsafe { Item::new(ptr) };

                        // SAFETY: Items own callbacks, so they must live for the duration of
                        // `self.ui`.
                        self.ui.alloc_object(item)
                    })
            }
        }
    };
}

impl_push_item_fn_with_name!(
    "
        Appends a new item with the given text and returns it.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    push_new_item,
    uiMenuAppendItem,
);

impl_push_item_fn_with_name!(
    "
        Appends a new check item with the given text and returns it.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    push_new_check_item,
    uiMenuAppendCheckItem,
);

impl_push_item_fn!(
    r#"
        Appends a new item that exits the application when clicked and returns it.

        The text of the item is "Quit" in English, and it is pre-configured with an
        [`Item::on_clicked`] callback to exit the application.

        It is idiomatic to append this item to a menu named "File".

        # Examples

        ```no_run
        // TODO
        ```
    "#,
    push_new_quit_item,
    uiMenuAppendQuitItem,
);

impl_push_item_fn!(
    r#"
        Appends a new item with the text "Preferences..." in English and returns it.

        This item is ideal for opening a preferences window that allows users to view and modify
        options related to the application's functionality. See [`Item::on_clicked`] for information
        on how this can be implemented.

        It is idiomatic to append this item to a menu named "Edit".

        # Examples

        ```no_run
        // TODO
        ```
    "#,
    push_new_preferences_item,
    uiMenuAppendPreferencesItem,
);

impl_push_item_fn!(
    r#"
        Appends a new item with the text "About" in English.

        This item is ideal for opening a window that tells users about the application when clicked.
        See [`Item::on_clicked`] for information on how this can be implemented.

        It is idiomatic to append this item to a menu named "Help".

        # Examples

        ```no_run
        // TODO
        ```
    "#,
    push_new_about_item,
    uiMenuAppendAboutItem,
);

impl Item<'_> {
    /// Creates a new [`Item`].
    ///
    /// # Safety
    ///
    /// `ptr` must point to a valid `uiMenuItem`.
    pub(super) unsafe fn new(ptr: *mut uiMenuItem) -> Self {
        Self {
            ptr,
            on_clicked: None,
        }
    }
}

// Menu items are *not* controls as they are not backed by a `uiControl`. Do not use them as such!
// See the note above the definition of [`Menu`] in *menu.rs* for more information.

/// An item within a [`Menu`].
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), boing::Error> {
/// use boing::Menu;
/// # use boing::Ui;
/// #
/// # let ui = Ui::new()?;
/// let menu: Menu;
/// # menu = ui.create_menu("")?;
///
/// let checkable = menu.push_new_item("Checkable")?;
/// checkable.set_checked(true);
/// checkable.on_clicked(|item| {
///     // Toggle the item.
///     item.set_checked(!item.is_checked());
/// });
///
/// let disabled = menu.push_new_item("Disabled")?;
/// disabled.disable();
/// #
/// # Ok(())
/// # }
/// ```
pub struct Item<'ui> {
    ptr: *mut uiMenuItem,
    on_clicked: Option<Box<dyn 'ui + FnMut(&mut Self)>>,
}

impl<'ui> Item<'ui> {
    bind_fn!(
        docs: "
            Makes this item interactable.

            Items are enabled by default.

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
            Makes this item uninteractable.

            The item may be greyed-out as a visual indicator.

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

            This callback is unset by default.

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

            Items are unchecked by default.

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

    /// A handle to the underlying *libui-ng* menu item object.
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
    pub(crate) fn as_ptr(&self) -> *mut uiMenuItem {
        self.ptr
    }
}
