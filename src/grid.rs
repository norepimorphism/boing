// SPDX-License-Identifier: MPL-2.0

//! [`Grid`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Grid`].
    pub fn create_grid(&self) -> Result<Grid, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewGrid() -> Grid,
        )
    }
}

def_subcontrol!(
    docs: "",
    ty: Grid,
    handle: uiGrid,
);

impl Grid {}
