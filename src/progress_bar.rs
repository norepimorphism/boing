// SPDX-License-Identifier: MPL-2.0

//! [`ProgressBar`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`ProgressBar`].
    pub fn create_progress_bar(&self) -> Result<ProgressBar, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewProgressBar() -> ProgressBar,
        )
    }
}

def_subcontrol!(
    docs: "",
    ty: ProgressBar,
    handle: uiProgressBar,
);

impl ProgressBar {
    pub fn set_value(&self, value: i32) {
        unsafe { uiProgressBarSetValue(self.as_ptr(), value) }
    }
}
