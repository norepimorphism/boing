// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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

def_subcontrol!(
    ty: Grid,
    handle: uiGrid,
);

impl Grid<'_> {

}
