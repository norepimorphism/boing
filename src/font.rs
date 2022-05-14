// SPDX-License-Identifier: MPL-2.0

//!

use std::mem::MaybeUninit;

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Picker`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn create_font_picker<'ui>(&'ui self) -> Result<&'ui mut Picker, crate::Error> {
        call_libui_new_fn!(
            ui: self,
            fn: uiNewFontButton() -> Picker,
        )
    }
}

def_subcontrol!(
    docs: "
        A pushbutton that opens a font picker dialog.

        # Examples

        ```no_run
        // TODO
        ```
    ",
    ty: Picker,
    handle: uiFontButton,
    cb_fns: [ on_selected<'a>() ],
);

/// A font returned by [`Picker::selected_font`].
pub struct Font {
    pub family: String,
    pub size: f64,
    pub weight: u32,
    pub italic_kind: Option<ItalicKind>,
    pub stretch: StretchKind,
}

pub enum ItalicKind {
    Oblique,
    True,
}

impl ItalicKind {
    fn try_from_desc(value: u32) -> Result<Option<Self>, ()> {
        match value {
            0 => Ok(None),
            1 => Ok(Some(Self::Oblique)),
            2 => Ok(Some(Self::True)),
            _ => Err(()),
        }
    }
}

pub enum StretchKind {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl StretchKind {
    fn try_from_desc(value: u32) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::UltraCondensed),
            1 => Ok(Self::ExtraCondensed),
            2 => Ok(Self::Condensed),
            3 => Ok(Self::SemiCondensed),
            4 => Ok(Self::Normal),
            5 => Ok(Self::SemiExpanded),
            6 => Ok(Self::ExtraExpanded),
            7 => Ok(Self::UltraExpanded),
            _ => Err(()),
        }
    }
}

impl<'a> Picker<'a> {
    /// The font currently selected by this picker.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // TODO
    /// ```
    pub fn selected_font(&self) -> Font {
        let mut desc = MaybeUninit::<uiFontDescriptor>::uninit();

        // TODO: This is UB if, for example, *libui-ng* doesn't initialize `desc` when a font hasn't
        // yet been selected. It would be safer to use default values instead of `MaybeUnunit`, but
        // that would probably necessitate a heap allocation.
        unsafe { uiFontButtonFont(self.as_ptr(), desc.as_mut_ptr()) };

        // SAFETY: *libui-ng* should've initialized it...
        let mut desc = unsafe { desc.assume_init() };

        // SAFETY: `desc.Family` should've been set by *libui-ng*...
        let family = unsafe { std::ffi::CStr::from_ptr(desc.Family) }.to_string_lossy().into();

        let size = desc.Size;
        let weight = desc.Weight;
        let italic_kind = ItalicKind::try_from_desc(desc.Italic).unwrap();
        let stretch = StretchKind::try_from_desc(desc.Stretch).unwrap();

        // Now that we've copied all the data from `desc`, we can safely free it.
        unsafe { uiFreeFontButtonFont(std::ptr::addr_of_mut!(desc)) };
        // To be sure, we now can no longer use this!
        drop(desc);

        Font {
            family,
            size,
            weight,
            italic_kind,
            stretch,
        }
    }

    bind_callback_fn!(
        docs: "
            Sets a callback for when a new font is selected.

            This callback is unset by default.

            # Examples

            ```no_run
            // TODO
            ```
        ",
        self: {
            ty: Picker<'a>,
            handle: uiFontButton,
            fn: on_selected(),
            cb: {
                sig: f -> (),
            },
        },
        libui: {
            fn: uiFontButtonOnChanged(),
            cb: {
                sig: () -> (),
            },
        },
    );
}
