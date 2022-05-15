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
    pub fn create_tab<'ui>(&'ui self) -> Result<&'ui mut Tab, crate::Error> {
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

        # Screenshots

        ## Windows

        ## macOS

        ## Linux
    ",
    ty: Tab,
    handle: uiTab,
);

impl Tab {
    bind_fn!(
        docs: "
            Deletes the page at the given index.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: remove_page(index: u16) },
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
        self: {
            fn: page_count() -> u16,
            map_out: |_, count| to_u16!(count),
        },
        libui: { fn: uiTabNumPages() },
    );

    bind_bool_fn!(
        docs: "
            Determines if the page at the given index is margined.

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
            Sets whether or not the page at the given index should be margined.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_page_margined(page: u16, value: bool) },
        libui: { fn: uiTabSetMargined() },
    );

    /// Appends a new page with the given name, returning its index.
    ///
    /// Note that the returned index may be invalidated after deletion operations. *boing* provides
    /// no mechanism to update the index in these cases, so it is the programmer's responsibility to
    /// keep track of each child's current index.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn push_new_page(
        &self,
        name: impl AsRef<str>,
        control: &mut impl DerefMut<Target = Control>,
    ) -> Result<u16, crate::Error> {
        let index = self.page_count();

        control.make_child();
        let name = make_cstring!(name.as_ref());
        unsafe { uiTabAppend(self.as_ptr(), name.as_ptr(), control.as_ptr()) };

        Ok(index)
    }

    /// Inserts a new page before the item at the given index, returning the index of the new page.
    ///
    /// Note that the returned index may be invalidated after deletion operations. *boing* provides
    /// no mechanism to update the index in these cases, so it is the programmer's responsibility to
    /// keep track of each child's current index.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn insert_new_page(
        &self,
        name: impl AsRef<str>,
        before: u16,
        control: &mut impl DerefMut<Target = Control>,
    ) -> Result<u16, crate::Error> {
        let index = self.page_count();

        control.make_child();
        let name = make_cstring!(name.as_ref());
        unsafe {
            uiTabInsertAt(
                self.as_ptr(),
                name.as_ptr(),
                before.into(),
                control.as_ptr(),
            )
        }

        Ok(index)
    }
}
