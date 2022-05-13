// SPDX-License-Identifier: MPL-2.0

//! [`Axis`].

use crate::prelude::*;

impl Ui {
    /// Creates a new horizontal [`Axis`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_horizontal_axis(&self) -> Result<Axis, crate::Error> {
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
    pub fn create_vertical_axis(&self) -> Result<Axis, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewVerticalBox() -> Axis,
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
    ty: Axis,
    handle: uiBox,
);

impl Axis {
    bind_bool_fn!(
        docs: "
            Determines if this axis is padded.

            # Examples

            ```no_run
            use boing::Axis;

            let axis: Axis;
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

            let x_axis: Axis;
            # let ui = boing::Ui::new().unwrap();
            # x_axis = ui.create_horizontal_axis().unwrap();

            // Axis-ception!
            for _ in 0..5 {
                let mut y_axis: Axis;
                # y_axis = ui.create_vertical_axis().unwrap();
                x_axis.append_child(&mut y_axis, false);
            }

            assert_eq!(5, x_axis.child_count());
            ```
        ",
        self: {
            fn: child_count() -> u16,
            map_out: |_, count| {
                assert_uint!(count);

                count as u16
            },
        },
        libui: { fn: uiBoxNumChildren() },
    );
}

pub struct Child {
    index: u16,
}

impl Axis {
    bind_fn!(
        docs: r#"
            Removes the child control at the given index.

            # Examples

            ```no_run
            use boing::Axis;

            let axis: Axis;
            # let ui = boing::Ui::new().unwrap();
            # axis = ui.create_horizontal_axis().unwrap();

            let mut progress_bar: boing::ProgressBar;
            # progress_bar = ui.create_progress_bar().unwrap();
            axis.append_child(&mut progress_bar, false);

            let mut button: boing::Pushbutton;
            # button = ui.create_pushbutton("").unwrap();
            axis.append_child(&mut button, false);

            // Remove the button from the axis.
            axis.delete_child(1);
            // Remove the progress bar from the axis.
            axis.delete_child(0);

            assert_eq!(0, axis.child_count());
            ```
        "#,
        self: { fn: delete_child(item: Child => |child: Child| child.index) },
        libui: { fn: uiBoxDelete() },
    );

    /// Inserts a child control at the zero-based index `self.child_count() - 1`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn append_new_child(
        &self,
        child: &mut impl DerefMut<Target = Control>,
        can_stretch: bool,
    ) -> Child {
        let index = self.child_count();

        child.make_child();
        unsafe { uiBoxAppend(self.as_ptr(), child.as_ptr(), can_stretch.into()) };

        Child { index }
    }
}
