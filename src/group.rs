// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Group`].
    pub fn create_group(&mut self, title: impl Into<Vec<u8>>) -> Result<Group, crate::Error> {
        let title = make_cstring!(title);
        call_libui_new_fn!(self, Group, uiNewGroup, title.as_ptr())
    }
}

def_subcontrol!(Group, uiGroup);

impl Group {
    pub fn set_child(&mut self, ui: &mut Ui, mut child: impl DerefMut<Target = Control>) {
        ui.remove_control(child.deref_mut().as_ptr());
        unsafe { uiGroupSetChild(self.as_ptr(), child.as_ptr()) };
    }
}
