// SPDX-License-Identifier: MPL-2.0

//! [`RadioButtons`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`RadioButtons`].
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


        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: RadioButtons,
    handle: uiRadioButtons,
    cb_fns: [ on_item_selected<'a>() ],
);

impl<'a> RadioButtons<'a> {
    bind_set_text_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: append_item(item_text) },
        libui: { fn: uiRadioButtonsAppend() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: selected_item() -> i32 },
        libui: { fn: uiRadioButtonsSelected() },
    );

    bind_fn!(
        docs: "
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
