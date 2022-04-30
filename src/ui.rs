pub mod control;

pub use control::Control;

use crate::prelude::*;
use std::{collections::HashSet, ffi::CStr, os::raw::c_char, ptr};

impl Ui {
    /// Runs *libui*.
    pub fn run(main: impl FnOnce(&mut Self)) -> Result<(), crate::Error> {
        let mut ui = Self::new()?;
        main(&mut ui);
        unsafe { uiMain() }

        Ok(())
    }

    /// Creates a new [`Ui`].
    fn new() -> Result<Self, crate::Error> {
        use std::sync::Once;

        static INIT: Once = Once::new();

        let mut result = Err(crate::Error::AlreadyInitedLibui);
        INIT.call_once(|| unsafe {
            result = Self::init_unchecked();
        });

        result.map(|_| Self {
            controls: HashSet::new(),
        })
    }

    unsafe fn init_unchecked() -> Result<(), crate::Error> {
        let mut init_options = uiInitOptions { Size: 0 };
        let err_msg = uiInit(ptr::addr_of_mut!(init_options));

        let result = Self::result_from_err_msg(err_msg);
        if result.is_err() {
            // It's OK to free `err_msg` now because we first copied its contents into `result`.
            uiFreeInitError(err_msg);
        }

        if let Err(ref cause) = result {
            // For some reason, on Windows, *libui* will return an error message starting with the
            // below string, which clearly indicates that no error has occurred. We catch this
            // special case and ignore it.
            if cause.starts_with("error initializing libui: initializing Common Controls; code 0") {
                return Ok(());
            }
        }

        result.map_err(|cause| crate::Error::LibuiFn {
            name: "uiInit",
            cause: Some(cause),
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

pub struct Ui {
    controls: HashSet<Control>,
}

impl Drop for Ui {
    fn drop(&mut self) {
        // Destroy all root controls (these are probably only windows).
        for control in self.controls.iter() {
            let control_ptr = control.as_ptr();

            unsafe {
                if (*control_ptr).Destroy.is_some() {
                    println!(
                        "Destroying: {:#?} ({:#?})",
                        control.as_ptr(),
                        control.type_id(),
                    );
                    uiControlDestroy(control_ptr);
                } else {
                    // TODO: Move this to control creation so that errors are caught earlier.
                    eprintln!(
                        "\
                        [libui-ng-sys] BUG: The control @ {:#?} requested its memory to be \
                        automatically freed when *libui-ng* quits, but its destructor is NULL. \
                        Please consider creating an issue on the *libui-ng-sys* GitHub! Thanks! \
                        ",
                        control_ptr,
                    );
                }
            }
        }

        unsafe { uiQuit() };
    }
}

impl Ui {
    pub(crate) unsafe fn add_control(
        &mut self,
        control: *mut uiControl,
        should_manage: bool,
    ) -> Control {
        let control = Control::from_ptr(control);
        if should_manage {
            let control: Control = std::mem::transmute_copy(&control);
            println!("Adding: {:#?} ({:#?})", control.as_ptr(), control.type_id());
            self.controls.insert(control);
        }

        control
    }

    pub(crate) fn release_control(&mut self, control: *mut uiControl) {
        let control = unsafe { Control::from_ptr(control) };
        self.controls.remove(&control);
    }
}
