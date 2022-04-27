#![windows_subsystem = "windows"]

//! A clone of the libui example program "libui Control Gallery".

fn main() {
    boing::Ui::run(|ui| {
        setup_menus(ui);

        let mut window = create_window(ui);

        let tab = create_tab(ui);
        window.set_child(ui, tab);

        window.show();
    })
    .unwrap();
}

fn setup_menus(ui: &mut boing::Ui) {
    setup_file_menu(ui);
    setup_edit_menu(ui);
    setup_help_menu(ui);
}

fn setup_file_menu(ui: &mut boing::Ui) {
    let mut menu = ui.create_menu("File").unwrap();
    // let _ = menu.append_item("Open").unwrap();
    // let _ = menu.append_item("Open Folder").unwrap();
    // let _ = menu.append_item("Save").unwrap();
    // let _ = menu.append_quit_item().unwrap();
}

fn setup_edit_menu(ui: &mut boing::Ui) {
    let mut menu = ui.create_menu("Edit").unwrap();
    // let _ = menu.append_check_item("Checkable Item").unwrap();
    // let _ = menu.append_item("Disabled Item").unwrap();
    // let _ = menu.append_preferences_item();
}

fn setup_help_menu(ui: &mut boing::Ui) {
    let mut menu = ui.create_menu("Help").unwrap();
    // let _ = menu.append_item("Help").unwrap();
    // let _ = menu.append_about_item();
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

    let basic_controls = create_basic_controls_page(ui);
    let _ = tab.append_page(ui, "Basic Controls", basic_controls).unwrap();

    let numbers_n_lists = create_numbers_n_lists_page(ui);
    let _ = tab.append_page(ui, "Numbers and Lists", numbers_n_lists).unwrap();

    let data_choosers = create_data_choosers_page(ui);
    let _ = tab.append_page(ui, "Data Choosers", data_choosers).unwrap();

    tab
}

fn create_basic_controls_page(ui: &mut boing::Ui) -> boing::Boxx {
    static LABEL_TEXT: &str = "This is a label. Right now, labels can only span one line.";

    let mut hbox = ui.create_horizontal_box().unwrap();
    hbox.set_padded(true);

    let button = ui.create_button("Button").unwrap();
    hbox.append_child(ui, button, true);

    let mut vbox = ui.create_vertical_box().unwrap();
    vbox.set_padded(true);
    vbox.append_child(ui, hbox, false);

    let label = ui.create_label(LABEL_TEXT).unwrap();
    vbox.append_child(ui, label, false);

    vbox
}

fn create_numbers_n_lists_page(ui: &mut boing::Ui) -> boing::Boxx {
    let mut hbox = ui.create_horizontal_box().unwrap();

    let numbers = create_numbers_group(ui);
    hbox.append_child(ui, numbers, true);

    let lists = create_lists_group(ui);
    hbox.append_child(ui, lists, true);

    hbox
}

fn create_numbers_group(ui: &mut boing::Ui) -> boing::Group {
    let mut group = ui.create_group("Numbers").unwrap();

    let vbox = create_numbers_vbox(ui);
    group.set_child(ui, vbox);

    group
}

fn create_numbers_vbox(ui: &mut boing::Ui) -> boing::Boxx {
    let mut vbox = ui.create_vertical_box().unwrap();

    let spinbox = ui.create_spinbox(0, 100).unwrap();
    vbox.append_child(ui, spinbox, false);

    let slider = ui.create_slider(0, 100).unwrap();
    vbox.append_child(ui, slider, false);

    let progress_bar = ui.create_progress_bar().unwrap();
    vbox.append_child(ui, progress_bar, false);

    let mut loading_bar = ui.create_progress_bar().unwrap();
    loading_bar.set_value(-1);
    vbox.append_child(ui, loading_bar, false);

    vbox
}

fn create_lists_group(ui: &mut boing::Ui) -> boing::Group {
    let mut group = ui.create_group("Lists").unwrap();

    let vbox = create_lists_vbox(ui);
    group.set_child(ui, vbox);

    group
}

fn create_lists_vbox(ui: &mut boing::Ui) -> boing::Boxx {
    ui.create_vertical_box().unwrap()
}

fn create_data_choosers_page(ui: &mut boing::Ui) -> boing::Boxx {
    let _vbox = ui.create_vertical_box().unwrap();
    let hbox = ui.create_horizontal_box().unwrap();

    hbox
}
