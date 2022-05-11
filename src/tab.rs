// SPDX-License-Identifier: MPL-2.0

//! [`Tab`].

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Tab`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_tab(&self) -> Result<Tab, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewTab() -> Tab,
        )
    }
}

def_subcontrol!(
    docs: "",
    ty: Tab,
    handle: uiTab,
);

impl Tab {
    /// Appends a page.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn append_page(
        &self,
        name: impl AsRef<str>,
        control: &mut impl DerefMut<Target = Control>,
    ) -> Result<(), crate::Error> {
        control.make_child();
        let name = make_cstring!(name.as_ref());
        unsafe { uiTabAppend(self.as_ptr(), name.as_ptr(), control.as_ptr()) };

        Ok(())
    }

    /// Inserts a page at the given index.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn insert_page(
        &self,
        name: impl AsRef<str>,
        index: u16,
        control: &mut impl DerefMut<Target = Control>,
    ) -> Result<(), crate::Error> {
        control.make_child();
        let name = make_cstring!(name.as_ref());
        unsafe { uiTabInsertAt(self.as_ptr(), name.as_ptr(), index.into(), control.as_ptr()) }

        Ok(())
    }

    bind_set_ty_fn!(
        docs: "Deletes the page represented by the given index.",
        self: { fn: delete_page(index: u16) },
        libui: { fn: uiTabDelete() },
    );

    bind_ty_fn!(
        docs: "The number of pages contained within this tab.",
        self: { fn: page_count() -> i32 },
        libui: { fn: uiTabNumPages() },
    );

    /// Determines if the page represented by the given index is margined.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn is_page_margined(&self, index: u16) -> bool {
        unsafe { uiTabMargined(self.as_ptr(), index.into()) == 1 }
    }

    /// Sets whether or not the page represented by the given index should be margined.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///  // TODO
    /// ```
    pub fn set_page_margined(&self, index: u16, value: bool) {
        unsafe { uiTabSetMargined(self.as_ptr(), index.into(), value.into()) }
    }
}
