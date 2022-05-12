// SPDX-License-Identifier: MPL-2.0

//! A graphical user interface provided by *libui-ng*.

use std::{ffi::CStr, os::raw::c_char, ptr};

use crate::prelude::*;

impl Ui {
    /// Creates a new [`Ui`].
    ///
    /// # Errors
    ///
    /// This function may only be called once. Calling [`Ui::new`] a second time will return
    /// [`crate::Error::AlreadyInitedLibui`].
    ///
    /// # Examples
    ///
    /// ```no_run, should_panic
    /// use boing::Ui;
    ///
    /// assert!(Ui::new().is_ok());
    ///
    /// // ERROR: *libui-ng* is already initialized.
    /// assert!(Ui::new().is_ok());
    /// ```
    pub fn new() -> Result<Self, crate::Error> {
        use std::sync::Once;

        static INIT: Once = Once::new();

        let mut result = Err(crate::Error::AlreadyInitedLibui);
        INIT.call_once(|| unsafe {
            // TODO: Calling `Self::init_unchecked` inside `INIT.call_once` prevents the caller from
            // retrying `Ui::new` if it fails the first time.
            result = Self::init_unchecked();
        });

        result.map(|_| {
            Self {
                arena: bumpalo::Bump::new(),
            }
        })
    }

    // Initializes *libui-ng* with the assumption that *libui-ng* is not already initialized.
    unsafe fn init_unchecked() -> Result<(), crate::Error> {
        // TODO: What is `uiInitOptions`? What does the `Size` field mean?
        let mut init_options = uiInitOptions { Size: 0 };

        let err_msg = uiInit(ptr::addr_of_mut!(init_options));

        let result = Self::result_from_err_msg(err_msg);
        if result.is_err() {
            // It's OK to free `err_msg` now because we first copied its contents into `result`.
            uiFreeInitError(err_msg);
        }

        if let Err(ref cause) = result {
            // For some reason, on Windows, *libui-ng* will sometimes return an error message
            // starting with the below string, which clearly indicates that no error has occurred.
            // We catch this special case and ignore it.
            if cause.starts_with("error initializing libui: initializing Common Controls; code 0") {
                return Ok(());
            }
        }

        result.map_err(|cause| {
            crate::Error::LibuiFn {
                name: "uiInit",
                cause: Some(cause),
            }
        })
    }

    fn result_from_err_msg(err_msg: *const c_char) -> Result<(), String> {
        if err_msg.is_null() {
            Ok(())
        } else {
            let err_msg = unsafe { CStr::from_ptr(err_msg) }
                .to_string_lossy()
                .into_owned();

            Err(err_msg)
        }
    }
}

/// A graphical user interface provided by *libui-ng*.
///
/// Access to a [`Ui`] object is necessary for creating widgets.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), boing::Error> {
/// use boing::Ui;
///
/// let ui = Ui::new()?;
///
/// let window = ui.create_window(
///     // Title.
///     "Hello World!",
///     // Width, in px.
///     640,
///     // Height, in px.
///     480,
///     // Does this window have a menubar?
///     false,
///     // Should this window quit the application when closed?
///     true,
/// )?;
///
/// window.show();
/// ui.run();
/// #
/// # Ok(())
/// # }
/// ```
pub struct Ui {
    // Spooky! Nearly nothing's here! As it turns out, [`Ui`] serves no functional purpose besides
    // instructing the compiler as to when it is valid for widgets to be created, used, and
    // destroyed, as well as store callback data for [`MenuItem`]s, which is the express purpose
    // for the `arena` field.
    arena: bumpalo::Bump,
}

impl Ui {
    /// Runs *libui-ng*.
    ///
    /// Due to the nature of this method entering the OS' main UI event loop, [`Ui::run`] does not
    /// return instantaneously; rather, it returns only once the user clicks a "Quit" menu item or
    /// closes a window created with `should_quit_on_close = true`.
    ///
    /// Although of little value, it is permissible to call this method multiple times.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), boing::Error> {
    /// use boing::Ui;
    ///
    /// let ui: Ui;
    /// # ui = Ui::new()?;
    ///
    /// // Muahahaha! Run forever! Force the user to close us with Task Manager!
    /// loop {
    ///     ui.run();
    /// }
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn run(&self) {
        unsafe { uiMain() };
    }

    /// Allocates an object.
    ///
    /// Wrap a value in this method when you need it to live for as long as [`Ui`].
    pub(crate) fn alloc_object<T>(&self, value: T) -> &mut T {
        self.arena.alloc(value)
    }
}
