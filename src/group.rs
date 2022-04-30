// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Group`].
    pub fn create_group(&mut self, title: impl AsRef<str>) -> Result<Group, crate::Error> {
        let title = make_cstring!(title.as_ref());
        call_libui_new_fn!(self, true, Group, uiNewGroup, title.as_ptr())
    }
}

def_subcontrol!(Group, uiGroup);

impl Group {
    bind_text_fn!(
        title,
        raw_title,
        title_ptr,
        uiGroupTitle,
    );

    bind_set_text_fn!(
        set_title,
        title,
        uiGroupSetTitle,
    );

    pub fn set_child(&mut self, ui: &mut Ui, mut child: impl DerefMut<Target = Control>) {
        ui.release_control(child.deref_mut().as_ptr());
        unsafe { uiGroupSetChild(self.as_ptr(), child.as_ptr()) };
    }

    bind_bool_fn!(
        is_margined,
        uiGroupMargined,
    );

    bind_set_bool_fn!(
        set_margined,
        uiGroupSetMargined,
    );
}
