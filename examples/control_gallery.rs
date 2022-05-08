//! A clone of the libui example program "libui Control Gallery".

// #![windows_subsystem = "windows"]

fn main() {
    setup_tracing();

    boing::Ui::run(|ui| {
        setup_menus(ui);

        let window = create_window(ui);
        window.set_child(create_tab(ui));
        window.show();
    })
    .unwrap();
}

fn setup_tracing() {
    tracing_subscriber::fmt().init();
}

fn setup_menus(ui: &boing::Ui) {
    setup_file_menu(ui);
    setup_edit_menu(ui);
    setup_help_menu(ui);
}

fn setup_file_menu(ui: &boing::Ui) {
    let menu = ui.create_menu("File").unwrap();
    menu.append_item(ui, "Open").unwrap();
    menu.append_item(ui, "Open Folder...").unwrap();
    menu.append_separator();
    menu.append_item(ui, "Save").unwrap();
    menu.append_item(ui, "Save As...").unwrap();
    menu.append_quit_item(ui).unwrap();
}

fn setup_edit_menu(ui: &boing::Ui) {
    let menu = ui.create_menu("Edit").unwrap();
    menu.append_check_item(ui, "Checkable").unwrap();
    menu.append_item(ui, "Disabled").unwrap();
    menu.append_preferences_item(ui).unwrap();
}

fn setup_help_menu(ui: &boing::Ui) {
    let menu = ui.create_menu("Help").unwrap();
    menu.append_item(ui, "Documentation").unwrap();

    let data = vec![1, 2, 3, 4];

    let about_item = menu.append_about_item(ui).unwrap();
    about_item.on_clicked(
        ui,
        || {
            println!("Sum: {}", data.iter().sum::<u32>());

            about_item.set_checked(true);

            let window = ui.create_window(
                "About libui Control Gallery",
                320,
                240,
                false,
                false,
            )
            .unwrap();
            window.show();
        },
    );
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
    tab.append_page("Basic Controls", create_basic_controls_page(ui)).unwrap();
    tab.append_page("Numbers and Lists", create_numbers_n_lists_page(ui)).unwrap();
    tab.append_page("Data Choosers", create_data_choosers_page(ui)).unwrap();

    tab
}

fn create_basic_controls_page(ui: &boing::Ui) -> &mut boing::UniBox {
    static LABEL_TEXT: &str = "This is a label. Right now, labels can only span one line.";

    let hbox = ui.create_horizontal_box().unwrap();
    hbox.set_padded(true);
    hbox.append_child(ui.create_button("Button").unwrap(), true);

    let vbox = ui.create_vertical_box().unwrap();
    vbox.set_padded(true);
    vbox.append_child(hbox, false);
    vbox.append_child(ui.create_label(LABEL_TEXT).unwrap(), false);

    vbox
}

fn create_numbers_n_lists_page(ui: &boing::Ui) -> &mut boing::UniBox {
    let hbox = ui.create_horizontal_box().unwrap();
    hbox.append_child(create_numbers_group(ui), true);
    hbox.append_child(create_lists_group(ui), true);

    hbox
}

fn create_numbers_group(ui: &boing::Ui) -> &mut boing::Group {
    let group = ui.create_group("Numbers").unwrap();
    group.set_child(create_numbers_vbox(ui));

    group
}

fn create_numbers_vbox(ui: &boing::Ui) -> &mut boing::UniBox {
    let vbox = ui.create_vertical_box().unwrap();
    vbox.append_child(ui.create_spinbox(0, 100).unwrap(), false);

    let slider = ui.create_slider(0, 100).unwrap();

    let data = vec![1, 2, 3, 4];
    slider.on_changed(ui, || println!("Slider sum: {}", data.iter().sum::<u32>()));

    vbox.append_child(slider, false);
    vbox.append_child(ui.create_progress_bar().unwrap(), false);

    let loading_bar = ui.create_progress_bar().unwrap();
    loading_bar.set_value(-1);
    vbox.append_child(loading_bar, false);

    vbox
}

fn create_lists_group(ui: &boing::Ui) -> &mut boing::Group {
    let group = ui.create_group("Lists").unwrap();
    group.set_child(create_lists_vbox(ui));

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
