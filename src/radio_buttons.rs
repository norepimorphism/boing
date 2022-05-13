// SPDX-License-Identifier: MPL-2.0

//! [`RadioButtons`].

use crate::prelude::*;

impl Ui {
    /// Creates a new set of [`RadioButtons`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_radio_buttons(&self) -> Result<RadioButtons, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewRadioButtons() -> RadioButtons,
        )
    }
}

def_subcontrol!(
    docs: "
        A set of mutually-exclusive selectable items.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: RadioButtons,
    handle: uiRadioButtons,
    cb_fns: [ on_item_selected<'a>() ],
    fields: [ item_count: u16 = 0 ],
);

pub struct Item {
    index: u16,
}

impl<'a> RadioButtons<'a> {
    bind_set_text_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self(mut): {
            fn: append_new_item(text) -> Item,
            map_out: |this: &mut Self, _| {
                // *libui-ng* doesn't provide a function to get the item count, so we have to keep
                // track ourselves.

                let index = this.item_count;
                this.item_count += 1;

                Item { index }
            },
        },
        libui: { fn: uiRadioButtonsAppend() },
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
        libui: { fn: uiRadioButtonsSelected() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_selected_item(item: Item => |item: Item| item.index) },
        libui: { fn: uiRadioButtonsSetSelected() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when an item is selected.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: RadioButtons<'a>,
            handle: uiRadioButtons,
            fn: on_item_selected(),
            cb: { sig: f -> () },
        },
        libui: {
            fn: uiRadioButtonsOnSelected(),
            cb: { sig: () -> () },
        },
    );
}
