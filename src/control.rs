// SPDX-License-Identifier: MPL-2.0

//! A type-erased control.

use std::{cell::Cell, os::raw::c_void};

use crate::prelude::*;

impl Control {
    pub(crate) fn new(ptr: *mut uiControl) -> Self {
        Self {
            ptr,
            is_child: Cell::new(false),
        }
    }
}

/// A type-erased control.
///
/// This type provides the underlying features that all controls must provide. When this type is
/// dropped, the control is destroyed, after which it may disappear for the user.
///
/// # Examples
///
/// ```no_run
///
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct Control {
    ptr: *mut uiControl,
    is_child: Cell<bool>,
}

impl Drop for Control {
    fn drop(&mut self) {
        if !self.is_child.get() {
            let ptr = self.as_ptr();
            tracing::debug!("Destroying control @ {:#?}", ptr);
            unsafe { uiControlDestroy(ptr) };
        }
    }
}

impl Control {
    /// A handle to the underlying *libui-ng* control object.
    ///
    /// # Safety
    ///
    /// The returned pointer is guaranteed to be non-null. Beyond that, it is your responsibility to
    /// use the handle appropriately. Consulting the *libui-ng* documentation or source code will be
    /// of utility in this regard, as well as the *boing* source code. See *[libui-ng-sys]* for
    /// *libui-ng* bindings.
    ///
    /// [libui-ng-sys]: https://github.com/norepimorphism/libui-ng-sys
    pub fn as_ptr(&self) -> *mut uiControl {
        self.ptr
    }

    pub(crate) fn make_child(&self) {
        self.is_child.set(true);
    }

    bind_bool_fn!(
        docs: "Determines if this control is visible.",
        self: { fn: is_visible() },
        libui: { fn: uiControlVisible() },
    );

    bind_bool_fn!(
        docs: "Determines if this control is interactable.",
        self: { fn: is_enabled() },
        libui: { fn: uiControlEnabled() },
    );

    // TODO: What does this function even do?
    bind_bool_fn!(
        docs: "Determines if this control is interactable to the user.",
        self: { fn: is_enabled_to_user() },
        libui: { fn: uiControlEnabledToUser() },
    );

    /// A handle to the underlying OS object.
    pub fn native_handle(&self) -> *mut c_void {
        unsafe { uiControlHandle(self.as_ptr()) as *mut c_void }
    }

    /// Makes this control visible.
    pub fn show(&self) {
        unsafe { uiControlShow(self.as_ptr()) };
    }

    /// Makes this control invisible.
    pub fn hide(&self) {
        unsafe { uiControlHide(self.as_ptr()) };
    }

    /// Makes this control interactable.
    pub fn enable(&self) {
        unsafe { uiControlEnable(self.as_ptr()) };
    }

    /// Makes this control uninteractable.
    pub fn disable(&self) {
        unsafe { uiControlDisable(self.as_ptr()) };
    }
}
