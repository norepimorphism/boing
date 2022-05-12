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
    docs: "


        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Tab,
    handle: uiTab,
);

impl Tab {
    bind_fn!(
        docs: "
            Deletes the page represented by the given index.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: delete_page(index: u16) },
        libui: { fn: uiTabDelete() },
    );

    bind_fn!(
        docs: "
            The number of pages contained within this tab.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: page_count() -> i32 },
        libui: { fn: uiTabNumPages() },
    );

    bind_bool_fn!(
        docs: "
            Determines if the page represented by the given index is margined.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: is_page_margined(index: u16) -> bool },
        libui: { fn: uiTabMargined() },
    );

    bind_fn!(
        docs: "
            Sets whether or not the page represented by the given index should be margined.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_page_margined(index: u16, value: bool) },
        libui: { fn: uiTabSetMargined() },
    );

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
}
