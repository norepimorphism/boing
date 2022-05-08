// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Checkbox`].

use crate::prelude::*;

def_subcontrol!(
    ty: Checkbox,
    handle: uiCheckbox,
    cb_fns: [
        on_toggled(),
    ],
);

impl<'ui> Checkbox<'ui> {
    bind_text_fn!(
        docs: "The text displayed next to this checkbox.",
        self: {
            fn: text,
            raw_fn: raw_text,
            as_ptr_fn: text_ptr,
        },
        libui: {
            fn: uiCheckboxText(),
        },
    );

    bind_set_text_fn!(
        docs: "Sets the text displayed next to this checkbox.",
        set_text,
        text,
        uiCheckboxSetText,
    );

    bind_callback_fn!(
        docs: "Sets a callback for when this checkbox is toggled.",
        self: {
            ty: Checkbox<'ui>,
            handle: uiCheckbox,
            fn: on_toggled(),
            cb: {
                sig: f -> (),
            },
        },
        libui: {
            fn: uiCheckboxOnToggled(),
            cb: {
                sig: () -> (),
            },
        },
    );

    bind_bool_fn!(
        docs: "Determines if this checkbox is checked.",
        checked,
        uiCheckboxChecked,
    );

    bind_set_bool_fn!(
        docs: "Sets whether or not this checkbox is checked.",
        set_checked,
        uiCheckboxSetChecked,
    );
}
