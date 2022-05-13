// SPDX-License-Identifier: MPL-2.0

//! [`Grid`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Grid`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_grid(&self) -> Result<Grid, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewGrid() -> Grid,
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
    ty: Grid,
    handle: uiGrid,
);

impl Grid {
    bind_bool_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: is_padded() -> bool },
        libui: { fn: uiGridPadded() },
    );

    bind_fn!(
        docs: "
            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_padded(value: bool) },
        libui: { fn: uiGridSetPadded() },
    );
}
