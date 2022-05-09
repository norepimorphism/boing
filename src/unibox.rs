// SPDX-License-Identifier: MPL-2.0

//! [`UniBox`].

use crate::prelude::*;

impl<'ui> Ui<'ui> {
    /// Creates a new horizontal [`UniBox`].
    pub fn create_horizontal_box<'a>(&'a self) -> Result<&'a mut UniBox<'ui>, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            ui_lt: 'ui,
            alloc: alloc_unibox,
            fn: uiNewHorizontalBox() -> UniBox,
        )
    }

    /// Creates a new vertical [`UniBox`].
    pub fn create_vertical_box<'a>(&'a self) -> Result<&'a mut UniBox<'ui>, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            ui_lt: 'ui,
            alloc: alloc_unibox,
            fn: uiNewVerticalBox() -> UniBox,
        )
    }
}

def_subcontrol!(
    docs: "",
    ty: UniBox,
    handle: uiBox,
);

impl<'ui> UniBox<'ui> {
    bind_bool_fn!(
        docs: "Determines if this box is padded.",
        is_padded,
        uiBoxPadded,
    );

    bind_set_bool_fn!(
        docs: "Sets whether or not this box is padded.",
        set_padded,
        uiBoxSetPadded,
    );

    pub fn append_child(
        &self,
        child: &mut impl DerefMut<Target = Control<'ui>>,
        can_stretch: bool,
    ) {
        child.make_child();
        unsafe { uiBoxAppend(self.as_ptr(), child.as_ptr(), can_stretch.into()) };
    }

    pub fn child_count(&self) -> i32 {
        unsafe { uiBoxNumChildren(self.as_ptr()) }
    }

    pub fn delete_child(&self, index: u16) {
        unsafe { uiBoxDelete(self.as_ptr(), index.into()) };
    }
}
