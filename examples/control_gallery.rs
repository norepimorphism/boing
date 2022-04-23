//! A clone of the libui example program "libui Control Gallery".

fn main() {
    let mut ui = boing::Ui::new().unwrap();

    setup_menus(&mut ui);

    let mut window = create_window(&mut ui);
    let tab = create_tab(&mut ui);

    window.set_child(&tab);

    window.show();

    ui.start();
}

fn setup_menus(ui: &mut boing::Ui) {
    setup_file_menu(ui);
    setup_edit_menu(ui);
    setup_help_menu(ui);
}

fn setup_file_menu(ui: &mut boing::Ui) {
    let mut menu = ui.create_menu("File").unwrap();
    let _ = menu.append_item("Open").unwrap();
    let _ = menu.append_item("Open Folder").unwrap();
    let _ = menu.append_item("Save").unwrap();
    let _ = menu.append_quit_item().unwrap();
}

fn setup_edit_menu(ui: &mut boing::Ui) {
    let mut menu = ui.create_menu("Edit").unwrap();
    let _ = menu.append_check_item("Checkable Item").unwrap();
    let _ = menu.append_item("Disabled Item").unwrap();
    let _ = menu.append_preferences_item();
}

fn setup_help_menu(ui: &mut boing::Ui) {
    let mut menu = ui.create_menu("Help").unwrap();
    let _ = menu.append_item("Help").unwrap();
    let _ = menu.append_about_item();
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
    let _ = tab.append_page(
        "Basic Controls",
        &create_basic_controls_page(ui),
    )
    .unwrap();
    let _ = tab.append_page(
        "Numbers and Lists",
        &create_numbers_page(ui),
    )
    .unwrap();
    let _ = tab.append_page(
        "Data Choosers",
        &create_data_choosers_page(ui),
    )
    .unwrap();

    tab
}

// These are guaranteed to crash lol

fn create_basic_controls_page(ui: &mut boing::Ui) -> boing::Boxx {
    let vbox = ui.create_vertical_box().unwrap();
    let _hbox = ui.create_horizontal_box().unwrap();

    vbox
}

fn create_numbers_page(ui: &mut boing::Ui) -> boing::Boxx {
    let _vbox = ui.create_vertical_box().unwrap();
    let hbox = ui.create_horizontal_box().unwrap();

    hbox
}

fn create_data_choosers_page(ui: &mut boing::Ui) -> boing::Boxx {
    let _vbox = ui.create_vertical_box().unwrap();
    let hbox = ui.create_horizontal_box().unwrap();

    hbox
}
