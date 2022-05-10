// SPDX-License-Identifier: MPL-2.0

//! [`Label`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Label`].
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
    docs: "",
    ty: Label,
    handle: uiLabel,
);

impl Label {
    bind_text_fn!(
        docs: "The text displayed by this label.",
        self: {
            fn: text,
            raw_fn: raw_text,
            as_ptr_fn: text_ptr,
        },
        libui: {
            fn: uiLabelText(),
        }
    );

    bind_set_text_fn!(
        docs: "Sets the text displayed by this label.",
        set_text,
        text,
        uiLabelSetText,
    );
}
