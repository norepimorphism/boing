// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Grid`].
    pub fn create_grid(&mut self) -> Result<Grid, crate::Error> {
        call_libui_new_fn!(Grid, uiNewGrid)
    }
}

def_subcontrol_with_ptr_ty!(Grid, uiGrid);
