// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

def_subcontrol!(Checkbox, uiCheckbox);

impl Checkbox {
    bind_text_fn!(
        text,
        raw_text,
        text_ptr,
        uiCheckboxText,
    );

    bind_set_text_fn!(
        set_text,
        text,
        uiCheckboxSetText,
    );

    bind_callback_fn!(
        on_toggled,
        uiCheckboxOnToggled;
        (),
        uiCheckbox,
    );

    bind_bool_fn!(
        checked,
        uiCheckboxChecked,
    );

    bind_set_bool_fn!(
        set_checked,
        uiCheckboxSetChecked,
    );
}
