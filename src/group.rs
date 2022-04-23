// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use std::ffi::CString;

impl Ui {
    /// Creates a new [`Group`].
    pub fn create_group(&mut self, title: impl Into<Vec<u8>>) -> Result<Group, crate::Error> {
        let title = CString::new(title).map_err(crate::Error::ConvertString)?;
        call_libui_new_fn!(Group, Group, title.as_ptr())
    }
}

def_subcontrol_with_ptr_ty!(Group, uiGroup);
