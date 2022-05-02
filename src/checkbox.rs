// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Checkbox`].

use crate::prelude::*;

def_subcontrol!(Checkbox, uiCheckbox);

impl Checkbox {
    bind_text_fn!(
        "The text displayed next to this checkbox.",
        text,
        raw_text,
        text_ptr,
        uiCheckboxText,
    );

    bind_set_text_fn!(
        "Sets the text displayed next to this checkbox.",
        set_text,
        text,
        uiCheckboxSetText,
    );

    bind_callback_fn!(
        "Sets a callback for when this checkbox is toggled.",
        Checkbox,
        on_toggled,
        uiCheckboxOnToggled;
        f -> (),
        (),
        : uiCheckbox,
    );

    bind_bool_fn!(
        "Determines if this checkbox is checked.",
        checked,
        uiCheckboxChecked,
    );

    bind_set_bool_fn!(
        "Sets whether or not this checkbox is checked.",
        set_checked,
        uiCheckboxSetChecked,
    );
}
