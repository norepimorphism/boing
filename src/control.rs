// SPDX-License-Identifier: MPL-2.0

//! [`Control`].

use std::{cell::Cell, marker::PhantomData, os::raw::c_void};

use crate::prelude::*;

impl<'ui> Control<'ui> {
    pub(crate) fn new(ptr: *mut uiControl) -> Self {
        Self {
            ptr,
            is_child: Cell::new(false),
            _ui: PhantomData,
        }
    }
}

/// A type-erased control.
///
/// This type provides the underlying features that all controls must provide. If you have access to
/// a [`Control`] but not the concrete control object, you can downcast with [`Control::downcast`].
#[derive(Debug, Eq, PartialEq)]
pub struct Control<'ui> {
    ptr: *mut uiControl,
    is_child: Cell<bool>,
    _ui: PhantomData<&'ui Ui<'ui>>,
}

impl Drop for Control<'_> {
    fn drop(&mut self) {
        if !self.is_child.get() {
            let ptr = self.as_ptr();
            tracing::debug!("Destroying control @ {:#?}", ptr);
            unsafe { uiControlDestroy(ptr) };
        }
    }
}

impl Control<'_> {
    /// A handle to the underlying *libui-ng* control object.
    pub fn as_ptr(&self) -> *mut uiControl {
        self.ptr
    }

    pub(crate) fn make_child(&self) {
        self.is_child.set(true);
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

        /// The concrete type of a [`Control`].
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum TypeId {
            $(
                #[doc = concat!("A [`crate::", stringify!($type), "`].")]
                $type,
            )*
            /// The control is of an unknown type.
            ///
            /// [`Control`]s with this type ID cannot be downcasted into a concrete control object.
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
    /// The concrete type of this control.
    pub fn type_id(&self) -> TypeId {
        TypeId::new(unsafe { (*self.ptr).TypeSignature })
    }
}

impl TypeId {
    /// Creates a new [`TypeId`] from a raw control type signature.
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
        docs: "Determines if this control is interactable.",
        is_enabled,
        uiControlEnabled,
    );

    // TODO: What does this function even do?
    bind_bool_fn!(
        docs: "Determines if this control is interactable to the user.",
        is_enabled_to_user,
        uiControlEnabledToUser,
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
