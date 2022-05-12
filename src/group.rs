// SPDX-License-Identifier: MPL-2.0

//! [`Group`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Group`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
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
    docs: "


        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Group,
    handle: uiGroup,
);

impl Group {
    bind_text_fn!(
        docs: "
            The title of this group.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: title(),
            raw_fn: raw_title(),
            as_ptr_fn: title_ptr(),
        },
        libui: { fn: uiGroupTitle() },
    );

    bind_set_text_fn!(
        docs: "
            Sets the title of this group.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_title(title) },
        libui: { fn: uiGroupSetTitle() },
    );

    bind_add_child_fn!(
        docs: "
            Sets the child control of this group.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_child<'ui>(child) },
        libui: { fn: uiGroupSetChild() },
    );

    bind_bool_fn!(
        docs: "
            Determines if this group has margins.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: is_margined() -> bool },
        libui: { fn: uiGroupMargined() },
    );

    bind_fn!(
        docs: "
            Sets whether or not this group has margins.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_margined(value: bool) },
        libui: { fn: uiGroupSetMargined() },
    );
}
