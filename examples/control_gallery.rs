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
        &create_numbers_n_lists_page(ui),
    )
    .unwrap();
    let _ = tab.append_page(
        "Data Choosers",
        &create_data_choosers_page(ui),
    )
    .unwrap();

    tab
}

fn create_basic_controls_page(ui: &mut boing::Ui) -> boing::Boxx {
    static LABEL_TEXT: &str = "This is a label. Right now, labels can only span one line.";

    let mut hbox = ui.create_horizontal_box().unwrap();
    hbox.set_padded(true);
    hbox.append_child(&ui.create_button("Button").unwrap(), false);

    let mut vbox = ui.create_vertical_box().unwrap();
    vbox.set_padded(true);
    vbox.append_child(&hbox, false);
    vbox.append_child(&ui.create_label(LABEL_TEXT).unwrap(), false);

    vbox
}

fn create_numbers_n_lists_page(ui: &mut boing::Ui) -> boing::Boxx {
    let mut hbox = ui.create_horizontal_box().unwrap();
    hbox.append_child(&create_numbers_group(ui), true);
    hbox.append_child(&create_lists_group(ui), true);

    hbox
}

fn create_numbers_group(ui: &mut boing::Ui) -> boing::Group {
    let mut numbers_group = ui.create_group("Numbers").unwrap();
    numbers_group.set_child(&create_numbers_vbox(ui));

    numbers_group
}

fn create_numbers_vbox(ui: &mut boing::Ui) -> boing::Boxx {
    let mut vbox = ui.create_vertical_box().unwrap();
    vbox.append_child(&ui.create_spinbox(0, 100).unwrap(), false);
    vbox.append_child(&ui.create_slider(0, 100).unwrap(), false);
    vbox.append_child(&ui.create_progress_bar().unwrap(), false);

    let mut loading_bar = ui.create_progress_bar().unwrap();
    loading_bar.set_value(-1);
    vbox.append_child(&loading_bar, false);

    vbox
}

fn create_lists_group(ui: &mut boing::Ui) -> boing::Group {
    let mut lists_group = ui.create_group("Lists").unwrap();
    lists_group.set_child(&create_lists_vbox(ui));

    lists_group
}

fn create_lists_vbox(ui: &mut boing::Ui) -> boing::Boxx {
    ui.create_vertical_box().unwrap()
}

fn create_data_choosers_page(ui: &mut boing::Ui) -> boing::Boxx {
    let _vbox = ui.create_vertical_box().unwrap();
    let hbox = ui.create_horizontal_box().unwrap();

    hbox
}
