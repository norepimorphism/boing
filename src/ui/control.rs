// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use std::os::raw::c_void;

#[derive(Eq, Hash, PartialEq)]
pub struct Control(*mut uiControl);

impl Control {
    pub(super) unsafe fn from_ptr(ptr: *mut uiControl) -> Self {
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

    pub fn type_id(&self) -> TypeId {
        TypeId::new(unsafe { (*self.0).TypeSignature })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeId {
    Area,
    UniBox,
    Button,
    Checkbox,
    ColorButton,
    Combobox,
    DateTimePicker,
    EditableCombobox,
    FormEntry,
    FontButton,
    Form,
    Grid,
    Group,
    Label,
    MultilineFormEntry,
    ProgressBar,
    RadioButtons,
    Separator,
    Slider,
    Spinbox,
    Tab,
    Table,
    Window,
    Unknown(u32),
}

impl TypeId {
    fn new(sig: u32) -> Self {
        match sig {
            uiAreaSignature => Self::Area,
            uiBoxSignature => Self::UniBox,
            uiButtonSignature => Self::Button,
            uiCheckboxSignature => Self::Checkbox,
            uiColorButtonSignature => Self::ColorButton,
            uiComboboxSignature => Self::Combobox,
            uiDateTimePickerSignature => Self::DateTimePicker,
            uiEditableComboboxSignature => Self::EditableCombobox,
            uiEntrySignature => Self::FormEntry,
            uiFontButtonSignature => Self::FontButton,
            uiFormSignature => Self::Form,
            uiGridSignature => Self::Grid,
            uiGroupSignature => Self::Group,
            uiLabelSignature => Self::Label,
            uiMultilineEntrySignature => Self::MultilineFormEntry,
            uiProgressBarSignature => Self::ProgressBar,
            uiRadioButtonsSignature => Self::RadioButtons,
            uiSeparatorSignature => Self::Separator,
            uiSliderSignature => Self::Slider,
            uiSpinboxSignature => Self::Spinbox,
            uiTabSignature => Self::Tab,
            uiTableSignature => Self::Table,
            uiWindowSignature => Self::Window,
            _ => Self::Unknown(sig),
        }
    }
}

pub enum Downcasted {
    Window(crate::Window),
}

impl Control {
    pub fn native_handle(&self) -> *mut c_void {
        unsafe { uiControlHandle(self.as_ptr()) as *mut c_void }
    }

    bind_bool_fn!(
        is_visible,
        uiControlVisible,
    );

    pub fn show(&mut self) {
        unsafe { uiControlShow(self.as_ptr()) };
    }

    pub fn hide(&mut self) {
        unsafe { uiControlHide(self.as_ptr()) };
    }

    bind_bool_fn!(
        is_enabled,
        uiControlEnabled,
    );

    pub fn enable(&mut self) {
        unsafe { uiControlEnable(self.as_ptr()) };
    }

    pub fn disable(&mut self) {
        unsafe { uiControlDisable(self.as_ptr()) };
    }

    bind_bool_fn!(
        is_enabled_to_user,
        uiControlEnabledToUser,
    );
}
