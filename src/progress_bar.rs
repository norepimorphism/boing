// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`ProgressBar`].
    pub fn create_progress_bar(&mut self) -> Result<ProgressBar, crate::Error> {
        call_libui_new_fn!(ProgressBar, ProgressBar,)
    }
}

def_subcontrol_with_ptr_ty!(ProgressBar, uiProgressBar);
