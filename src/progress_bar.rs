// SPDX-License-Identifier: MPL-2.0

//! A horizontal bar that continuously fills as an action progresses.

use crate::prelude::*;

impl Ui {
    /// Creates a new [`ProgressBar`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_progress_bar<'ui>(&'ui self) -> Result<&'ui mut ProgressBar, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewProgressBar() -> ProgressBar,
        )
    }
}

def_subcontrol!(
    docs: "
        A horizontal bar that continuously fills as an action progresses.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: ProgressBar,
    handle: uiProgressBar,
);

impl ProgressBar {
    bind_fn!(
        docs: "
            The current value of this progress bar.

            This is 0 by default.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            fn: value() -> u16,
            map_out: |_, value| to_u16!(value),
        },
        libui: { fn: uiProgressBarValue() },
    );

    bind_fn!(
        docs: "
            Sets the value of this progress bar.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_value(value: u16) },
        libui: { fn: uiProgressBarSetValue() },
    );

    bind_fn!(
        docs: "
            Marks this progress bar as being indefinite.

            For many GUI toolkits, this causes the progress bar to continuously rotate a colored
            inner bar, indicating that an action is in progress but it is unknown when the action
            will complete.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: { fn: set_as_indefinite() },
        libui: { fn: uiProgressBarSetValue(-1) },
    );
}
