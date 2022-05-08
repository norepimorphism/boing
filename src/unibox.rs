// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [`UniBox`].

use crate::prelude::*;

impl ui!() {
    /// Creates a new horizontal [`UniBox`].
    pub fn create_horizontal_box(&self) -> Result<&mut UniBox, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            alloc: alloc_unibox,
            fn: uiNewHorizontalBox() -> UniBox,
        )
    }

    /// Creates a new vertical [`UniBox`].
    pub fn create_vertical_box(&self) -> Result<&mut UniBox, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            alloc: alloc_unibox,
            fn: uiNewVerticalBox() -> UniBox,
        )
    }
}

def_subcontrol!(
    ty: UniBox,
    handle: uiBox,
);

impl UniBox {
    pub fn append_child(
        &self,
        child: &mut impl DerefMut<Target = Control>,
        can_stretch: bool,
    ) {
        let child = std::mem::ManuallyDrop::new(child);
        unsafe { uiBoxAppend(self.as_ptr(), child.as_ptr(), can_stretch.into()) };
    }

    pub fn child_count(&self) -> i32 {
        unsafe { uiBoxNumChildren(self.as_ptr()) }
    }

    pub fn delete_child(&self, index: u16) {
        unsafe { uiBoxDelete(self.as_ptr(), index.into()) };
    }

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
}
