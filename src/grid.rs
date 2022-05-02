// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Grid`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Grid`].
    pub fn create_grid(&self) -> Result<&mut Grid, crate::Error> {
        call_libui_new_fn!(self, Grid, uiNewGrid)
    }
}

def_subcontrol!(Grid, uiGrid);

impl Grid {

}
