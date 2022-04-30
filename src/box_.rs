// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Uniaxial and biaxial boxes.

use crate::prelude::*;

impl Ui {
    /// Creates a new horizontal box.
    pub fn create_horizontal_box(&mut self) -> Result<UniBox, crate::Error> {
        call_libui_new_fn!(self, true, UniBox, uiNewHorizontalBox)
    }

    /// Creates a new vertical box.
    pub fn create_vertical_box(&mut self) -> Result<UniBox, crate::Error> {
        call_libui_new_fn!(self, true, UniBox, uiNewVerticalBox)
    }
}

def_subcontrol!(UniBox, uiBox);

impl UniBox {
    fn append_child_by_ref(
        &mut self,
        ui: &mut Ui,
        child: &mut impl DerefMut<Target = Control>,
        can_stretch: bool,
    ) {
        ui.release_control(child.deref_mut().as_ptr());
        unsafe { uiBoxAppend(self.as_ptr(), child.as_ptr(), can_stretch.into()) };
    }

    pub fn append_child(
        &mut self,
        ui: &mut Ui,
        mut child: impl DerefMut<Target = Control>,
        can_stretch: bool,
    ) {
        self.append_child_by_ref(ui, &mut child, can_stretch);
    }

    pub fn child_count(&self) -> i32 {
        unsafe { uiBoxNumChildren(self.as_ptr()) }
    }

    pub fn delete_child(&mut self, index: u16) {
        unsafe { uiBoxDelete(self.as_ptr(), index.into()) };
    }

    pub fn is_padded(&self) -> bool {
        unsafe { uiBoxPadded(self.as_ptr()) == 1 }
    }

    pub fn set_padded(&mut self, value: bool) {
        unsafe { uiBoxSetPadded(self.as_ptr(), value.into()) };
    }
}

impl Ui {
    /// Creates a new [`BiBox`].
    pub fn create_biaxial_box(&mut self, can_stretch: bool) -> Result<BiBox, crate::Error> {
        let mut hori = self.create_horizontal_box()?;
        let mut vert = self.create_vertical_box()?;
        hori.append_child_by_ref(self, &mut vert, can_stretch);

        Ok(BiBox {
            outer: hori,
            inner: vert,
            can_stretch,
        })
    }
}

pub struct BiBox {
    outer: UniBox,
    inner: UniBox,
    can_stretch: bool,
}

impl Deref for BiBox {
    type Target = Control;

    fn deref(&self) -> &Self::Target {
        self.outer.deref()
    }
}

impl DerefMut for BiBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.outer.deref_mut()
    }
}

impl BiBox {
    pub fn append_child(
        &mut self,
        ui: &mut Ui,
        child: impl DerefMut<Target = Control>,
    ) {
        self.inner.append_child(ui, child, self.can_stretch)
    }

    pub fn child_count(&self) -> i32 {
        self.inner.child_count()
    }

    pub fn delete_child(&mut self, index: u16) {
        self.inner.delete_child(index);
    }

    pub fn set_padded(&mut self, value: bool) {
        self.inner.set_padded(value);
        self.outer.set_padded(value);
    }
}
