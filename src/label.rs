// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Label`].

use crate::prelude::*;

impl<'ui> Ui<'ui> {
    /// Creates a new [`Label`].
    pub fn create_label<'a>(
        &'a self,
        text: impl AsRef<str>,
    ) -> Result<&'a mut Label<'ui>, crate::Error> {
        let text = make_cstring!(text.as_ref());

        call_libui_new_fn!(
            ui: self,
            ui_lt: 'ui,
            alloc: alloc_label,
            fn: uiNewLabel(text.as_ptr()) -> Label,
        )
    }
}

def_subcontrol!(ty: Label, handle: uiLabel,);

impl Label<'_> {
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
