// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;

pub struct Control(*mut uiControl);

impl Control {
    pub unsafe fn from_ptr(ptr: *mut uiControl) -> Self {
        Self(ptr)
    }

    pub fn as_ptr(&self) -> *mut uiControl {
        self.0
    }

    pub fn downcast(self) -> () {
        todo!()
    }

    pub fn is_visible(&self) -> bool {
        unsafe { uiControlVisible(self.0) == 1 }
    }

    pub fn show(&mut self) {
        unsafe { uiControlShow(self.0) }
    }

    pub fn hide(&mut self) {
        unsafe { uiControlHide(self.0) }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { uiControlEnabled(self.0) == 1}
    }

    pub fn enable(&mut self) {
        unsafe { uiControlEnable(self.0) }
    }

    pub fn disable(&mut self) {
        unsafe {  uiControlDisable(self.0) }
    }
}

impl Drop for Control {
    fn drop(&mut self) {
        unsafe { uiControlDestroy(self.0) }
    }
}
