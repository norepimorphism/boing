// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`ProgressBar`].

use crate::prelude::*;

impl<'ui> Ui<'ui> {
    /// Creates a new [`ProgressBar`].
    pub fn create_progress_bar<'a>(&'a self) -> Result<&'a mut ProgressBar<'ui>, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            ui_lt: 'ui,
            alloc: alloc_progress_bar,
            fn: uiNewProgressBar() -> ProgressBar,
        )
    }
}

def_subcontrol!(
    ty: ProgressBar,
    handle: uiProgressBar,
);

impl ProgressBar<'_> {
    pub fn set_value(&self, value: i32) {
        unsafe { uiProgressBarSetValue(self.as_ptr(), value) }
    }
}
