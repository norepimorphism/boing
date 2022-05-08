// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`Tab`].

use crate::prelude::*;
use std::mem::ManuallyDrop;

impl ui!() {
    /// Creates a new [`Tab`].
    pub fn create_tab(&self) -> Result<&mut Tab, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            alloc: alloc_tab,
            fn: uiNewTab() -> Tab,
        )
    }
}

def_subcontrol!(
    ty: Tab,
    handle: uiTab,
);

impl Tab {
    /// Appends a page.
    pub fn append_page(
        &self,
        name: impl AsRef<str>,
        control: &mut impl DerefMut<Target = Control>,
    ) -> Result<(), crate::Error> {
        let control = ManuallyDrop::new(control);
        let name = make_cstring!(name.as_ref());
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
        &self,
        name: impl AsRef<str>,
        index: u16,
        control: &mut impl DerefMut<Target = Control>,
    ) -> Result<(), crate::Error> {
        let control = ManuallyDrop::new(control);
        let name = make_cstring!(name.as_ref());
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
    pub fn delete_page(&self, index: u16) {
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
    pub fn set_page_margined(&self, index: u16, value: bool) {
        unsafe { uiTabSetMargined(self.as_ptr(), index.into(), value.into()) }
    }
}
