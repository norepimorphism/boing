// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`ProgressBar`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`ProgressBar`].
    pub fn create_progress_bar(&self) -> Result<&mut ProgressBar, crate::Error> {
        call_libui_new_fn!(self, ProgressBar, uiNewProgressBar)
    }
}

def_subcontrol!(ProgressBar, uiProgressBar);

impl ProgressBar {
    pub fn set_value(&self, value: i32) {
        unsafe { uiProgressBarSetValue(self.as_ptr(), value) }
    }
}
