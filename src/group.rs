// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Group`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Group`].
    pub fn create_group(&self, title: impl AsRef<str>) -> Result<&mut Group, crate::Error> {
        let title = make_cstring!(title.as_ref());
        call_libui_new_fn!(self, Group, uiNewGroup, title.as_ptr())
    }
}

def_subcontrol!(Group, uiGroup);

impl Group {
    bind_text_fn!(
        "The title of this group.",
        title,
        raw_title,
        title_ptr,
        uiGroupTitle,
    );

    bind_set_text_fn!(
        "Sets the title of this group.",
        set_title,
        title,
        uiGroupSetTitle,
    );

    bind_add_child_fn!(
        "Sets the child control of this group.",
        set_child,
        child,
        uiGroupSetChild,
    );

    bind_bool_fn!(
        "Determines if this group has margins.",
        is_margined,
        uiGroupMargined,
    );

    bind_set_bool_fn!(
        "Sets whether or not this group has margins.",
        set_margined,
        uiGroupSetMargined,
    );
}
