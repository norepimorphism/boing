// SPDX-License-Identifier: MPL-2.0

//! A toggleable button with adjacent customizable text.

use crate::prelude::*;

def_subcontrol!(
    docs: "
        A toggleable button with adjacent customizable text.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Checkbox,
    handle: uiCheckbox,
    cb_fns: [ on_toggled<'a>() ],
);

impl<'a> Checkbox<'a> {
    bind_text_fn!(
        docs: "
            The text displayed next to this checkbox.

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
        libui: { fn: uiCheckboxText() },
    );

    bind_set_text_fn!(
        docs: "
            Sets the text displayed next to this checkbox.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_text(text) },
        libui: { fn: uiCheckboxSetText() },
    );

    bind_callback_fn!(
        docs: "
            Sets a callback for when this checkbox is toggled.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: Checkbox<'a>,
            handle: uiCheckbox,
            fn: on_toggled(),
            cb: { sig: f -> () },
        },
        libui: {
            fn: uiCheckboxOnToggled(),
            cb: { sig: () -> () },
        },
    );

    bind_bool_fn!(
        docs: "
            Determines if this checkbox is checked.


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: checked() -> bool },
        libui: { fn: uiCheckboxChecked() },
    );

    bind_fn!(
        docs: "
            Sets whether or not this checkbox is checked.


            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_checked(value: bool) },
        libui: { fn: uiCheckboxSetChecked() },
    );
}
