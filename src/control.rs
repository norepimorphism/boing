// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use std::os::raw::c_void;

pub struct Control(*mut uiControl);

impl Control {
    pub unsafe fn from_ptr(ptr: *mut uiControl) -> Self {
        Self(ptr)
    }

    pub fn as_ptr(&self) -> *mut uiControl {
        self.0
    }

    pub fn downcast(&self) -> Option<Downcasted> {
        match self.type_id() {
            _ => todo!(),
        }
    }

    fn type_id(&self) -> u32 {
        unsafe { (*self.0).Signature }
    }
}

pub enum Downcasted {
    Window(crate::Window),
}

impl Control {
    pub fn native_handle(&self) -> *mut c_void {
        unsafe { uiControlHandle(self.as_ptr()) as *mut c_void }
    }

    pub fn is_visible(&self) -> bool {
        unsafe { uiControlVisible(self.as_ptr()) == 1 }
    }

    pub fn show(&mut self) {
        unsafe { uiControlShow(self.as_ptr()) }
    }

    pub fn hide(&mut self) {
        unsafe { uiControlHide(self.as_ptr()) }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { uiControlEnabled(self.as_ptr()) == 1}
    }

    pub fn enable(&mut self) {
        unsafe { uiControlEnable(self.as_ptr()) }
    }

    pub fn disable(&mut self) {
        unsafe { uiControlDisable(self.as_ptr()) }
    }

    pub fn is_enabled_to_user(&self) -> bool {
        unsafe { uiControlEnabledToUser(self.as_ptr()) == 1 }
    }
}
