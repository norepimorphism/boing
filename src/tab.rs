// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use std::{ffi::CString, ptr};

impl Ui {
    /// Creates a new [`Tab`].
    pub fn create_tab(&mut self) -> Result<Tab, crate::Error> {
        call_libui_new_fn!(Tab, Tab,)
    }
}

def_subcontrol_with_ptr_ty!(Tab, uiTab);

impl Tab {
    pub fn append_page(
        &mut self,
        name: impl Into<Vec<u8>>,
        control: Control,
    ) -> Result<(), crate::Error> {
        let name = CString::new(name).map_err(crate::Error::ConvertString)?;
        unsafe { uiTabAppend(self.as_ptr(), name.as_ptr(), control.as_ptr()) };

        Ok(())
    }

    /// The number of pages contained within this tab.
    pub fn page_count(&self) -> i32 {
        unsafe { uiTabNumPages(self.as_ptr()) }
    }

    /// Determines if the page represented by the given index is margined.
    pub fn is_page_margined(&self, index: u16) -> bool {
        unsafe { uiTabMargined(self.as_ptr(), index.into()) == 1}
    }
}
