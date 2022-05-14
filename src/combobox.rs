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
    cb_fns: [ on_item_selected<'a>() ],
);

pub struct Item {
    index: u16,
}

impl<'a> Combobox<'a> {
    bind_set_text_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: append_new_item(text) -> Item,
            map_out: |this: &Self, _| {
                let index = this.item_count();

                Item { index }
            },
        },
        libui: { fn: uiComboboxAppend() },
    );

    bind_set_text_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: insert_new_item(
                text,
                ^before: Item => |item: Item| item.index,
            ) -> Item,
            map_out: |_: &Self, _: ()| {
                Item { index: before }
            },
        },
        libui: { fn: uiComboboxInsertAt() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: delete_item(item: Item => |item: Item| item.index) },
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
        self: {
            fn: item_count() -> u16,
            map_out: |_, count| {
                assert_uint!(count);

                count as u16
            },
        },
        libui: { fn: uiComboboxNumItems() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: selected_item() -> Item,
            map_out: |_, index| {
                assert_uint!(index);

                Item { index: index as u16 }
            },
        },
        libui: { fn: uiComboboxSelected() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_item_selected(item: Item => |item: Item| item.index) },
        libui: { fn: uiComboboxSetSelected() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for an item is selected.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: Combobox<'a>,
            handle: uiCombobox,
            fn: on_item_selected(),
            cb: {
                sig: f -> (),
            },
        },
        libui: {
            fn: uiComboboxOnSelected(),
            cb: {
                sig: () -> (),
            },
        },
    );
}
