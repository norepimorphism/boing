// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use std::{marker::PhantomData, os::raw::c_void};

impl<'ui> Control<'ui> {
    pub(crate) fn new(ptr: *mut uiControl) -> Self {
        Self {
            ptr,
            _ui: PhantomData,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Control<'ui> {
    ptr: *mut uiControl,
    _ui: PhantomData<&'ui Ui<'ui>>,
}

impl Drop for Control<'_> {
    fn drop(&mut self) {
        // TODO: unsafe { uiControlDestroy(self.as_ptr()) };
    }
}

impl Control<'_> {
    pub fn as_ptr(&self) -> *mut uiControl {
        self.ptr
    }
}

macro_rules! impl_downcast {
    ($($type:ident)*) => {
        impl<'ui> Control<'ui> {
            pub fn downcast(self) -> Option<Downcasted<'ui>> {
                match self.type_id() {
                    $(
                        TypeId::$type => {
                            Some(Downcasted::$type($crate::$type::from_control(self)))
                        }
                    )*
                    TypeId::Unknown(_) => None,
                }
            }
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum TypeId {
            $(
                $type,
            )*
            Unknown(u32),
        }

        pub enum Downcasted<'ui> {
            $(
                $type($crate::$type<'ui>),
            )*
        }
    };
}

impl_downcast! {
    Area
    UniBox
    Button
    Checkbox
    // ColorButton
    Combobox
    // DateTimePicker
    // EditableCombobox
    // FormEntry
    // FontButton
    Form
    Grid
    Group
    Label
    // MultilineFormEntry
    ProgressBar
    // RadioButtons
    // Separator
    Slider
    Spinbox
    Tab
    Table
    Window
}

impl Control<'_> {
    pub fn type_id(&self) -> TypeId {
        TypeId::new(unsafe { (*self.ptr).TypeSignature })
    }
}

impl TypeId {
    fn new(sig: u32) -> Self {
        match sig {
            uiAreaSignature => Self::Area,
            uiBoxSignature => Self::UniBox,
            uiButtonSignature => Self::Button,
            uiCheckboxSignature => Self::Checkbox,
            // uiColorButtonSignature => Self::ColorButton,
            uiComboboxSignature => Self::Combobox,
            // uiDateTimePickerSignature => Self::DateTimePicker,
            // uiEditableComboboxSignature => Self::EditableCombobox,
            // uiEntrySignature => Self::FormEntry,
            // uiFontButtonSignature => Self::FontButton,
            uiFormSignature => Self::Form,
            uiGridSignature => Self::Grid,
            uiGroupSignature => Self::Group,
            uiLabelSignature => Self::Label,
            // uiMultilineEntrySignature => Self::MultilineFormEntry,
            uiProgressBarSignature => Self::ProgressBar,
            // uiRadioButtonsSignature => Self::RadioButtons,
            // uiSeparatorSignature => Self::Separator,
            uiSliderSignature => Self::Slider,
            uiSpinboxSignature => Self::Spinbox,
            uiTabSignature => Self::Tab,
            uiTableSignature => Self::Table,
            uiWindowSignature => Self::Window,
            _ => Self::Unknown(sig),
        }
    }
}

impl Control<'_> {
    bind_bool_fn!(
        docs: "Determines if this control is visible.",
        is_visible,
        uiControlVisible,
    );

    bind_bool_fn!(
        docs: "Determines if this control is enabled.",
        is_enabled,
        uiControlEnabled,
    );

    bind_bool_fn!(
        docs: "Determines if this control is enabled to the user.",
        is_enabled_to_user,
        uiControlEnabledToUser,
    );

    pub fn native_handle(&self) -> *mut c_void {
        unsafe { uiControlHandle(self.as_ptr()) as *mut c_void }
    }

    pub fn show(&self) {
        unsafe { uiControlShow(self.as_ptr()) };
    }

    pub fn hide(&self) {
        unsafe { uiControlHide(self.as_ptr()) };
    }

    pub fn enable(&self) {
        unsafe { uiControlEnable(self.as_ptr()) };
    }

    pub fn disable(&self) {
        unsafe { uiControlDisable(self.as_ptr()) };
    }
}
