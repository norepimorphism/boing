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
    pub fn create_combobox<'ui>(&'ui self) -> Result<&'ui mut Combobox, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewCombobox() -> Combobox,
        )
    }
}

def_subcontrol!(
    docs: "
        A drop-down menu of mutually-exclusive selectable items.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Combobox,
    handle: uiCombobox,
    cb_fns: [ on_item_selected<'a>() ],
);

impl<'a> Combobox<'a> {
    bind_set_text_fn!(
        docs: "
            Appends a new item with the given text, returning its index.

            Note that the returned index may be invalidated after insertion or deletion operations.
            *libui-ng* provides no mechanism to update the index in these cases, so it is the
            programmer's responsibility to keep track of each child's current index.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: push_new_item(text) -> u16,
            map_out: |this: &Self, _| {
                // We subtract one as `this.item_count()` is the item count *after* the new item was
                // appended. For example, if the count was originally 0 but was increased to 1 after
                // the new item was appended, then the index is 1 - 1 = 0.
                this.item_count() - 1
            },
        },
        libui: { fn: uiComboboxAppend() },
    );

    bind_set_text_fn!(
        docs: "
            Inserts a new item with the given text before the item at the given index, returning the
            index of the new item.

            Note that the returned index may be invalidated after insertion or deletion operations.
            *boing* provides no mechanism to update the index in these cases, so it is the
            programmer's responsibility to keep track of each child's current index.

            # Arguments

            The returned index is equivalent to `before`.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: insert_new_item(text, ^before: u16) -> u16,
            map_out: |_: &Self, _: ()| before,
        },
        libui: { fn: uiComboboxInsertAt() },
    );

    bind_fn!(
        docs: "
            Removes the item at the given index.

            This action may invalidate the previous indices of other items.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: remove_item(index: u16) },
        libui: { fn: uiComboboxDelete() },
    );

    bind_fn!(
        docs: "
            Clears the combobox, removing all items.

            [`Combobox::item_count`] should return 0 after this method is called.

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
            The number of items this combobox contains.

            This is 0 initially.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: item_count() -> u16,
            map_out: |_, count| to_u16!(count),
        },
        libui: { fn: uiComboboxNumItems() },
    );

    bind_fn!(
        docs: "
            The index of the currently-selected item.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: selected_item() -> u16,
            map_out: |_, index| to_u16!(index),
        },
        libui: { fn: uiComboboxSelected() },
    );

    bind_fn!(
        docs: "
            Selects the item with the given index.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: select_item(item: u16) },
        libui: { fn: uiComboboxSetSelected() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for an item is selected.

            This callback is unset by default.

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
