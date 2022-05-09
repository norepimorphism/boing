// SPDX-License-Identifier: MPL-2.0

//! [`Grid`].

use crate::prelude::*;

impl<'ui> Ui<'ui> {
    /// Creates a new [`Grid`].
    pub fn create_grid<'a>(&'a self) -> Result<&'a mut Grid<'ui>, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            ui_lt: 'ui,
            alloc: alloc_grid,
            fn: uiNewGrid() -> Grid,
        )
    }
}

def_subcontrol!(ty: Grid, handle: uiGrid,);

impl Grid<'_> {}
