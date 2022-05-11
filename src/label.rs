// SPDX-License-Identifier: MPL-2.0

//! [`Label`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Label`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_label(
        &self,
        text: impl AsRef<str>,
    ) -> Result<Label, crate::Error> {
        let text = make_cstring!(text.as_ref());

        call_libui_new_fn!(
            ui: self,
            fn: uiNewLabel(text.as_ptr()) -> Label,
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
    ty: Label,
    handle: uiLabel,
);

impl Label {
    bind_text_fn!(
        docs: "
            The text displayed by this label.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: text(),
            raw_fn: raw_text(),
            as_ptr_fn: text_ptr(),
        },
        libui: { fn: uiLabelText() },
    );

    bind_set_text_fn!(
        docs: "
            Sets the text displayed by this label.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_text(text) },
        libui: { fn: uiLabelSetText() },
    );
}
