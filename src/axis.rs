// SPDX-License-Identifier: MPL-2.0

//! A series of controls aligned to a common line.

use crate::prelude::*;

impl Ui {
    /// Creates a new horizontal [`Axis`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_horizontal_axis<'ui>(&'ui self) -> Result<&'ui mut Axis, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewHorizontalBox() -> Axis,
        )
    }

    /// Creates a new vertical [`Axis`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_vertical_axis<'ui>(&'ui self) -> Result<&'ui mut Axis, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewVerticalBox() -> Axis,
        )
    }
}

def_subcontrol!(
    docs: "
        A series of controls aligned to a common line.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Axis,
    handle: uiBox,
);

impl Axis {
    bind_bool_fn!(
        docs: "
            Determines if this axis is padded.

            Axes are unpadded by default.

            # Examples

            ```no_run
            use boing::Axis;

            let axis: &mut Axis;
            # let ui = boing::Ui::new().unwrap();
            # axis = ui.create_horizontal_axis().unwrap();
            assert!(!axis.is_padded());

            axis.set_padded(true);
            assert!(axis.is_padded());
            ```
        ",
        self: { fn: is_padded() -> bool },
        libui: { fn: uiBoxPadded() },
    );

    bind_fn!(
        docs: "
            Sets whether or not this axis is padded.

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
            The number of child controls this axis contains.

            # Examples

            ```no_run
            use boing::Axis;

            let x_axis: &mut Axis;
            # let ui = boing::Ui::new().unwrap();
            # x_axis = ui.create_horizontal_axis().unwrap();

            // Axis-ception!
            for _ in 0..5 {
                let y_axis: &mut Axis;
                # y_axis = ui.create_vertical_axis().unwrap();
                x_axis.push_new_child(y_axis, false);
            }

            assert_eq!(5, x_axis.child_count());
            ```
        ",
        self: {
            fn: child_count() -> u16,
            map_out: |_, count| to_u16!(count),
        },
        libui: { fn: uiBoxNumChildren() },
    );
}

impl Axis {
    bind_fn!(
        docs: r#"
            Removes the child control at the given index.

            This action may invalidate the previous indices of other items.

            # Examples

            ```no_run
            use boing::Axis;

            let axis: &mut Axis;
            # let ui = boing::Ui::new().unwrap();
            # axis = ui.create_horizontal_axis().unwrap();

            let mut progress_bar: &mut boing::ProgressBar;
            # progress_bar = ui.create_progress_bar().unwrap();
            let progress_bar_idx = axis.push_new_child(progress_bar, false);

            let mut button: &mut boing::Pushbutton;
            # button = ui.create_pushbutton("").unwrap();
            let button_idx = axis.push_new_child(button, false);

            // Remove the button from the axis.
            axis.remove_child(button_idx);
            // Remove the progress bar from the axis.
            axis.remove_child(progress_bar_idx);

            assert_eq!(0, axis.child_count());
            ```
        "#,
        self: { fn: remove_child(index: u16) },
        libui: { fn: uiBoxDelete() },
    );

    /// Appends a new child control, returning its index.
    ///
    /// Note that the returned index may be invalidated after deletion operations. *boing* provides
    /// no mechanism to update the index in these cases, so it is the programmer's responsibility to
    /// keep track of each child's current index.
    ///
    /// # Arguments
    ///
    /// When `can_stretch` is `true`, the child control will fill the axis' container. Otherwise,
    /// controls retain their original size.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn push_new_child(
        &self,
        child: &mut impl DerefMut<Target = Control>,
        can_stretch: bool,
    ) -> u16 {
        let index = self.child_count();

        child.make_child();
        unsafe { uiBoxAppend(self.as_ptr(), child.as_ptr(), can_stretch.into()) };

        index
    }
}
