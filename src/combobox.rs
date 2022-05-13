// SPDX-License-Identifier: MPL-2.0

//! [`Combobox`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Combobox`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_combobox(&self) -> Result<Combobox, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewCombobox() -> Combobox,
        )
    }
}

def_subcontrol!(
    docs: "


        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Combobox,
    handle: uiCombobox,
);

impl Combobox {
    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: delete_item(index: u16) },
        libui: { fn: uiComboboxDelete() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: clear() },
        libui: { fn: uiComboboxClear() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: item_count() -> i32 },
        libui: { fn: uiComboboxNumItems() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: selected_item() -> i32 },
        libui: { fn: uiComboboxSelected() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_item_selected(index: u16) },
        libui: { fn: uiComboboxSetSelected() },
    );
}
