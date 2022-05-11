// SPDX-License-Identifier: MPL-2.0

//! [`UniBox`].

use crate::prelude::*;

impl Ui {
    /// Creates a new horizontal [`UniBox`].
    pub fn create_horizontal_box(&self) -> Result<UniBox, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewHorizontalBox() -> UniBox,
        )
    }

    /// Creates a new vertical [`UniBox`].
    pub fn create_vertical_box(&self) -> Result<UniBox, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewVerticalBox() -> UniBox,
        )
    }
}

def_subcontrol!(
    docs: "",
    ty: UniBox,
    handle: uiBox,
);

impl UniBox {
    bind_bool_fn!(
        docs: "Determines if this box is padded.",
        self: { fn: is_padded() },
        libui: { fn: uiBoxPadded() },
    );

    bind_set_bool_fn!(
        docs: "Sets whether or not this box is padded.",
        self: { fn: set_padded() },
        libui: { fn: uiBoxSetPadded() },
    );

    pub fn append_child(
        &self,
        child: &mut impl DerefMut<Target = Control>,
        can_stretch: bool,
    ) {
        child.make_child();
        unsafe { uiBoxAppend(self.as_ptr(), child.as_ptr(), can_stretch.into()) };
    }

    bind_ty_fn!(
        docs: "",
        self: { fn: child_count() -> i32 },
        libui: { fn: uiBoxNumChildren() },
    );

    bind_set_ty_fn!(
        docs: "",
        self: { fn: delete_child(index: u16) },
        libui: { fn: uiBoxDelete() },
    );
}
