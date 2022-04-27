// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Tab`].
    pub fn create_tab(&mut self) -> Result<Tab, crate::Error> {
        call_libui_new_fn!(self, true, Tab, uiNewTab)
    }
}

def_subcontrol!(Tab, uiTab);

impl Tab {
    /// Appends a page.
    pub fn append_page(
        &mut self,
        ui: &mut Ui,
        name: impl Into<Vec<u8>>,
        mut control: impl DerefMut<Target = Control>,
    ) -> Result<(), crate::Error> {
        ui.release_control(control.deref_mut().as_ptr());
        let name = make_cstring!(name);
        unsafe {
            uiTabAppend(
                self.as_ptr(),
                name.as_ptr(),
                control.as_ptr(),
            )
        };

        Ok(())
    }

    /// Inserts a page at the given index.
    pub fn insert_page(
        &mut self,
        ui: &mut Ui,
        name: impl Into<Vec<u8>>,
        index: u16,
        mut control: impl DerefMut<Target = Control>,
    ) -> Result<(), crate::Error> {
        ui.release_control(control.deref_mut().as_ptr());
        let name = make_cstring!(name);
        unsafe {
            uiTabInsertAt(
                self.as_ptr(),
                name.as_ptr(),
                index.into(),
                control.as_ptr(),
            )
        }

        Ok(())
    }

    /// Deletes the page represented by the given index.
    pub fn delete_page(&mut self, index: u16) {
        unsafe { uiTabDelete(self.as_ptr(), index.into()) }
    }

    /// The number of pages contained within this tab.
    pub fn page_count(&self) -> i32 {
        unsafe { uiTabNumPages(self.as_ptr()) }
    }

    /// Determines if the page represented by the given index is margined.
    pub fn is_page_margined(&self, index: u16) -> bool {
        unsafe { uiTabMargined(self.as_ptr(), index.into()) == 1 }
    }

    /// Sets whether or not the page represented by the given index should be margined.
    pub fn set_page_margined(&mut self, index: u16, value: bool) {
        unsafe { uiTabSetMargined(self.as_ptr(), index.into(), value.into()) }
    }
}
