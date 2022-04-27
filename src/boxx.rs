// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    /// Creates a new horizontal [`Boxx`].
    pub fn create_horizontal_box(&mut self) -> Result<Boxx, crate::Error> {
        call_libui_new_fn!(self, true, Boxx, uiNewHorizontalBox)
    }

    /// Creates a new vertical [`Boxx`].
    pub fn create_vertical_box(&mut self) -> Result<Boxx, crate::Error> {
        call_libui_new_fn!(self, true, Boxx, uiNewVerticalBox)
    }
}

def_subcontrol!(Boxx, uiBox);

impl Boxx {
    pub fn append_child(
        &mut self,
        ui: &mut Ui,
        mut child: impl DerefMut<Target = Control>,
        can_stretch: bool,
    ) {
        ui.release_control(child.deref_mut().as_ptr());
        unsafe { uiBoxAppend(self.as_ptr(), child.as_ptr(), can_stretch.into()) };
    }

    pub fn child_count(&self) -> i32 {
        unsafe { uiBoxNumChildren(self.as_ptr()) }
    }

    pub fn delete_child(&mut self, index: u16) {
        unsafe { uiBoxDelete(self.as_ptr(), index.into()) };
    }

    pub fn is_padded(&self) -> bool {
        unsafe { uiBoxPadded(self.as_ptr()) == 1 }
    }

    pub fn set_padded(&mut self, value: bool) {
        unsafe { uiBoxSetPadded(self.as_ptr(), value.into()) };
    }
}
