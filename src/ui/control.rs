// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use std::os::raw::c_void;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Control(*mut uiControl);

impl Control {
    pub(super) unsafe fn from_ptr(ptr: *mut uiControl) -> Self {
        Self(ptr)
    }
}

impl Drop for Control {
    fn drop(&mut self) {
        unsafe { uiControlDestroy(self.as_ptr()) };
    }
}

impl Control {
    pub fn as_ptr(&self) -> *mut uiControl {
        self.0
    }
}

macro_rules! impl_downcast {
    ($($type:ident)*) => {
        impl Control {
            pub fn downcast(self) -> Option<Downcasted> {
                use $crate::ui::FromControl as _;

                match self.type_id() {
                    $(
                        TypeId::$type => unsafe {
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

        #[derive(Debug, Eq, PartialEq)]
        pub enum Downcasted {
            $(
                $type($crate::$type),
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

impl Control {
    pub fn type_id(&self) -> TypeId {
        TypeId::new(unsafe { (*self.0).TypeSignature })
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

impl Control {
    pub fn native_handle(&self) -> *mut c_void {
        unsafe { uiControlHandle(self.as_ptr()) as *mut c_void }
    }

    bind_bool_fn!(
        "Determines if this control is visible.",
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
        "Determines if this control is enabled.",
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
        "Determines if this control is enabled to the user.",
        is_enabled_to_user,
        uiControlEnabledToUser,
    );
}
