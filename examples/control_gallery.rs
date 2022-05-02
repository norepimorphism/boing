//! A clone of the libui example program "libui Control Gallery".

#![windows_subsystem = "windows"]

fn main() {
    boing::Ui::run(|ui| {
        setup_menus(ui);

        let window = create_window(ui);

        let tab = create_tab(ui);
        window.set_child(tab);

        window.show();
    })
    .unwrap();
}

fn setup_menus(ui: &boing::Ui) {
    setup_file_menu(ui);
    setup_edit_menu(ui);
    setup_help_menu(ui);
}

fn setup_file_menu(ui: &boing::Ui) {
    let mut menu = ui.create_menu("File").unwrap();
    menu.append_item("Open").unwrap();
    menu.append_item("Open Folder...").unwrap();
    menu.append_separator();
    menu.append_item("Save").unwrap();
    menu.append_item("Save As...").unwrap();
    menu.append_quit_item().unwrap();
}

fn setup_edit_menu(ui: &boing::Ui) {
    let mut menu = ui.create_menu("Edit").unwrap();
    menu.append_check_item("Checkable").unwrap();
    menu.append_item("Disabled").unwrap();
    menu.append_preferences_item().unwrap();
}

fn setup_help_menu(ui: &boing::Ui) {
    let mut menu = ui.create_menu("Help").unwrap();
    menu.append_item("Documentation").unwrap();
    menu.append_about_item().unwrap();
}

fn create_window(ui: &boing::Ui) -> &mut boing::Window {
    let window = ui.create_window(
        "libui Control Gallery",
        640,
        480,
        true,
        true,
    )
    .unwrap();

    window.set_margined(true);

    window
}

fn create_tab(ui: &boing::Ui) -> &mut boing::Tab {
    let tab = ui.create_tab().unwrap();

    let basic_controls = create_basic_controls_page(ui);
    let _ = tab.append_page("Basic Controls", basic_controls).unwrap();

    let numbers_n_lists = create_numbers_n_lists_page(ui);
    let _ = tab.append_page("Numbers and Lists", numbers_n_lists).unwrap();

    let data_choosers = create_data_choosers_page(ui);
    let _ = tab.append_page("Data Choosers", data_choosers).unwrap();

    tab
}

fn create_basic_controls_page(ui: &boing::Ui) -> &mut boing::UniBox {
    static LABEL_TEXT: &str = "This is a label. Right now, labels can only span one line.";

    let hbox = ui.create_horizontal_box().unwrap();
    hbox.set_padded(true);

    let button = ui.create_button("Button").unwrap();
    hbox.append_child(button, true);

    let vbox = ui.create_vertical_box().unwrap();
    vbox.set_padded(true);
    vbox.append_child(hbox, false);

    let label = ui.create_label(LABEL_TEXT).unwrap();
    vbox.append_child(label, false);

    vbox
}

fn create_numbers_n_lists_page(ui: &boing::Ui) -> &mut boing::UniBox {
    let hbox = ui.create_horizontal_box().unwrap();

    let numbers = create_numbers_group(ui);
    hbox.append_child(numbers, true);

    let lists = create_lists_group(ui);
    hbox.append_child(lists, true);

    hbox
}

fn create_numbers_group(ui: &boing::Ui) -> &mut boing::Group {
    let group = ui.create_group("Numbers").unwrap();

    let vbox = create_numbers_vbox(ui);
    group.set_child(vbox);

    group
}

fn create_numbers_vbox(ui: &boing::Ui) -> &mut boing::UniBox {
    let vbox = ui.create_vertical_box().unwrap();

    let spinbox = ui.create_spinbox(0, 100).unwrap();
    vbox.append_child(spinbox, false);

    let slider = ui.create_slider(0, 100).unwrap();
    vbox.append_child(slider, false);

    let progress_bar = ui.create_progress_bar().unwrap();
    vbox.append_child(progress_bar, false);

    let loading_bar = ui.create_progress_bar().unwrap();
    loading_bar.set_value(-1);
    vbox.append_child(loading_bar, false);

    vbox
}

fn create_lists_group(ui: &boing::Ui) -> &mut boing::Group {
    let group = ui.create_group("Lists").unwrap();

    let vbox = create_lists_vbox(ui);
    group.set_child(vbox);

    group
}

fn create_lists_vbox(ui: &boing::Ui) -> &mut boing::UniBox {
    ui.create_vertical_box().unwrap()
}

fn create_data_choosers_page(ui: &boing::Ui) -> &mut boing::UniBox {
    let _vbox = ui.create_vertical_box().unwrap();
    let hbox = ui.create_horizontal_box().unwrap();

    hbox
}
