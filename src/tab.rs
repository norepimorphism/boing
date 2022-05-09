// SPDX-License-Identifier: MPL-2.0

//! [`Tab`].

use std::mem::ManuallyDrop;

use crate::prelude::*;

impl<'ui> Ui<'ui> {
    /// Creates a new [`Tab`].
    pub fn create_tab<'a>(&'a self) -> Result<&'a mut Tab<'ui>, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            ui_lt: 'ui,
            alloc: alloc_tab,
            fn: uiNewTab() -> Tab,
        )
    }
}

def_subcontrol!(ty: Tab, handle: uiTab,);

impl<'ui> Tab<'ui> {
    /// Appends a page.
    pub fn append_page(
        &self,
        name: impl AsRef<str>,
        control: &mut impl DerefMut<Target = Control<'ui>>,
    ) -> Result<(), crate::Error> {
        let control = ManuallyDrop::new(control);
        let name = make_cstring!(name.as_ref());
        unsafe { uiTabAppend(self.as_ptr(), name.as_ptr(), control.as_ptr()) };

        Ok(())
    }

    /// Inserts a page at the given index.
    pub fn insert_page(
        &self,
        name: impl AsRef<str>,
        index: u16,
        control: &mut impl DerefMut<Target = Control<'ui>>,
    ) -> Result<(), crate::Error> {
        let control = ManuallyDrop::new(control);
        let name = make_cstring!(name.as_ref());
        unsafe { uiTabInsertAt(self.as_ptr(), name.as_ptr(), index.into(), control.as_ptr()) }

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
