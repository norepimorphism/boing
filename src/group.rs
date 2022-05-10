// SPDX-License-Identifier: MPL-2.0

//! [`Group`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Group`].
    pub fn create_group(
        &self,
        title: impl AsRef<str>,
    ) -> Result<Group, crate::Error> {
        let title = make_cstring!(title.as_ref());

        call_libui_new_fn!(
            ui: self,
            fn: uiNewGroup(title.as_ptr()) -> Group,
        )
    }
}

def_subcontrol!(
    docs: "",
    ty: Group,
    handle: uiGroup,
);

impl<'ui> Group {
    bind_text_fn!(
        docs: "The title of this group.",
        self: {
            fn: title,
            raw_fn: raw_title,
            as_ptr_fn: title_ptr,
        },
        libui: {
            fn: uiGroupTitle(),
        },
    );

    bind_set_text_fn!(
        docs: "Sets the title of this group.",
        set_title,
        title,
        uiGroupSetTitle,
    );

    bind_add_child_fn!(
        docs: "Sets the child control of this group.",
        self: {
            fn: set_child<'ui>,
            child: child,
        },
        libui: {
            fn: uiGroupSetChild,
        },
    );

    bind_bool_fn!(
        docs: "Determines if this group has margins.",
        is_margined,
        uiGroupMargined,
    );

    bind_set_bool_fn!(
        docs: "Sets whether or not this group has margins.",
        set_margined,
        uiGroupSetMargined,
    );
}
