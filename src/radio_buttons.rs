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
    pub fn create_radio_buttons<'ui>(&'ui self) -> Result<&'ui mut RadioButtons, crate::Error> {
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

impl<'a> RadioButtons<'a> {
    bind_set_text_fn!(
        docs: "
            Appends a new item with the given text, returning its index.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self(mut): {
            fn: push_new_item(text) -> u16,
            map_out: |this: &mut Self, _| {
                // *libui-ng* doesn't provide a function to get the item count, so we have to keep
                // track ourselves.

                let index = this.item_count;
                this.item_count += 1;

                index
            },
        },
        libui: { fn: uiRadioButtonsAppend() },
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
        libui: { fn: uiRadioButtonsSelected() },
    );

    bind_fn!(
        docs: "
            Selects the item with the given index.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_selected_item(index: u16) },
        libui: { fn: uiRadioButtonsSetSelected() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when an item is selected.

            This callback is unset by default.

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
