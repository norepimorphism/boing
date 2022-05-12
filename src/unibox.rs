// SPDX-License-Identifier: MPL-2.0

//! [`UniBox`].

use crate::prelude::*;

impl Ui {
    /// Creates a new horizontal [`UniBox`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_horizontal_box(&self) -> Result<UniBox, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewHorizontalBox() -> UniBox,
        )
    }

    /// Creates a new vertical [`UniBox`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_vertical_box(&self) -> Result<UniBox, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewVerticalBox() -> UniBox,
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
    ty: UniBox,
    handle: uiBox,
);

impl UniBox {
    bind_bool_fn!(
        docs: "
            Determines if this box is padded.

            # Examples

            ```no_run
            use boing::UniBox;

            let unibox: UniBox;
            # let ui = boing::Ui::new().unwrap();
            # unibox = ui.create_horizontal_box().unwrap();
            assert!(!unibox.is_padded());

            unibox.set_padded(true);
            assert!(unibox.is_padded());
            ```
        ",
        self: { fn: is_padded() -> bool },
        libui: { fn: uiBoxPadded() },
    );

    bind_fn!(
        docs: "
            Sets whether or not this box is padded.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_padded(value: bool) },
        libui: { fn: uiBoxSetPadded() },
    );

    bind_fn!(
        docs: "
            The number of child controls this box contains.

            # Examples

            ```no_run
            use boing::UniBox;

            let outer: UniBox;
            # let ui = boing::Ui::new().unwrap();
            # outer = ui.create_horizontal_box().unwrap();

            // Box-ception!
            for _ in 0..5 {
                let mut inner: UniBox;
                # inner = ui.create_horizontal_box().unwrap();
                outer.append_child(&mut inner, false);
            }

            assert_eq!(5, outer.child_count());
            ```
        ",
        self: { fn: child_count() -> i32 },
        libui: { fn: uiBoxNumChildren() },
    );

    bind_fn!(
        docs: r#"
            Removes the child control at the given index.

            # Examples

            ```no_run
            use boing::UniBox;

            let unibox: UniBox;
            # let ui = boing::Ui::new().unwrap();
            # unibox = ui.create_horizontal_box().unwrap();

            let mut progress_bar: boing::ProgressBar;
            # progress_bar = ui.create_progress_bar().unwrap();
            unibox.append_child(&mut progress_bar, false);

            let mut button: boing::Button;
            # button = ui.create_button("").unwrap();
            unibox.append_child(&mut button, false);

            // Remove the button from the box.
            unibox.delete_child(1);
            // Remove the progress bar from the box.
            unibox.delete_child(0);

            assert_eq!(0, unibox.child_count());
            ```
        "#,
        self: { fn: delete_child(index: u16) },
        libui: { fn: uiBoxDelete() },
    );

    /// Inserts a child control at the zero-based index `self.child_count() - 1`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn append_child(&self, child: &mut impl DerefMut<Target = Control>, can_stretch: bool) {
        child.make_child();
        unsafe { uiBoxAppend(self.as_ptr(), child.as_ptr(), can_stretch.into()) };
    }
}
