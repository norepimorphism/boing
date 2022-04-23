//! A clone of the libui example program "libui Control Gallery".

fn main() {
    let mut ui = boing::Ui::new().unwrap();

    let _file_menu = create_file_menu(&mut ui);
    let _edit_menu = create_edit_menu(&mut ui);
    let _help_menu = create_help_menu(&mut ui);

    let mut window = create_window(&mut ui);
    let tab = create_tab(&mut ui);

    window.set_child(&tab);

    window.show();

    ui.start();
}

fn create_file_menu(ui: &mut boing::Ui) -> boing::Menu {
    ui.create_menu("File").unwrap()
}

fn create_edit_menu(ui: &mut boing::Ui) -> boing::Menu {
    ui.create_menu("Edit").unwrap()
}

fn create_help_menu(ui: &mut boing::Ui) -> boing::Menu {
    ui.create_menu("Help").unwrap()
}

fn create_window(ui: &mut boing::Ui) -> boing::Window {
    let mut window = ui.create_window(
        "libui Control Gallery",
        640,
        480,
        true,
    )
    .unwrap();

    window.set_resizeable(false);
    window.set_margined(true);

    window
}

fn create_tab(ui: &mut boing::Ui) -> boing::Tab {
    let mut tab = ui.create_tab().unwrap();

    tab.append_page("Basic Controls", create_basic_controls_page()).unwrap();
    tab.append_page("Numbers and Lists", create_numbers_page()).unwrap();
    tab.append_page("Data Choosers", create_data_choosers_page()).unwrap();

    tab
}

// These are guaranteed to crash lol

fn create_basic_controls_page() -> boing::Control {
    unsafe { boing::Control::from_ptr(std::ptr::null_mut()) }
}

fn create_numbers_page() -> boing::Control {
    unsafe { boing::Control::from_ptr(std::ptr::null_mut()) }
}

fn create_data_choosers_page() -> boing::Control {
    unsafe { boing::Control::from_ptr(std::ptr::null_mut()) }
}
