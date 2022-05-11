// SPDX-License-Identifier: MPL-2.0

//! A clone of the libui example program "libui Control Gallery".

// #![windows_subsystem = "windows"]

fn main() {
    setup_tracing();

    let ui = boing::Ui::new().unwrap();
    menubar::setup(&ui);

    let window = ui
        .create_window("libui Control Gallery", 640, 480, true, true)
        .unwrap();
    window.set_margined(true);

    let mut tab = tab::create(&ui);
    window.set_child(&mut tab);
    window.show();

    ui.run();
}

fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}

mod menubar {
    pub fn setup(ui: &boing::Ui) {
        setup_file_menu(ui);
        setup_edit_menu(ui);
        setup_help_menu(ui);
    }

    fn setup_file_menu<'ui>(ui: &'ui boing::Ui) {
        let menu = ui.create_menu("File").unwrap();
        menu.append_item(ui, "Open").unwrap();
        menu.append_item(ui, "Open Folder...").unwrap();
        menu.append_separator();
        menu.append_item(ui, "Save").unwrap();
        menu.append_item(ui, "Save As...").unwrap();
        menu.append_quit_item(ui).unwrap();
    }

    fn setup_edit_menu<'ui>(ui: &'ui boing::Ui) {
        let menu = ui.create_menu("Edit").unwrap();

        let checkable = menu.append_item(ui, "Checkable").unwrap();
        checkable.set_checked(true);
        checkable.on_clicked(|item| {
            // Toggle the checkbox.
            item.set_checked(!item.is_checked());
        });

        let disabled = menu.append_item(ui, "Disabled").unwrap();
        disabled.disable();

        menu.append_preferences_item(ui).unwrap();
    }
    fn setup_help_menu<'ui>(ui: &'ui boing::Ui) {
        let menu = ui.create_menu("Help").unwrap();
        menu.append_item(ui, "Documentation").unwrap();
        menu.append_about_item(ui).unwrap();
    }
}

mod tab {
    pub fn create(ui: &boing::Ui) -> boing::Tab {
        let tab = ui.create_tab().unwrap();
        tab.append_page("Basic Controls", &mut create_basic_controls_page(ui))
            .unwrap();
        tab.append_page("Numbers and Lists", &mut create_numbers_n_lists_page(ui))
            .unwrap();
        tab.append_page("Data Choosers", &mut create_data_choosers_page(ui))
            .unwrap();

        tab
    }

    fn create_basic_controls_page(ui: &boing::Ui) -> boing::UniBox {
        static LABEL_TEXT: &str = "This is a label. Right now, labels can only span one line.";

        let mut hbox = ui.create_horizontal_box().unwrap();
        hbox.set_padded(true);
        hbox.append_child(&mut ui.create_button("Button").unwrap(), true);

        let vbox = ui.create_vertical_box().unwrap();
        vbox.set_padded(true);
        vbox.append_child(&mut hbox, false);
        vbox.append_child(&mut ui.create_label(LABEL_TEXT).unwrap(), false);

        vbox
    }

    fn create_numbers_n_lists_page(ui: &boing::Ui) -> boing::UniBox {
        let hbox = ui.create_horizontal_box().unwrap();
        hbox.append_child(&mut create_numbers_group(ui), true);
        hbox.append_child(&mut create_lists_group(ui), true);

        hbox
    }

    fn create_numbers_group(ui: &boing::Ui) -> boing::Group {
        let group = ui.create_group("Numbers").unwrap();
        group.set_child(&mut create_numbers_vbox(ui));

        group
    }

    fn create_numbers_vbox(ui: &boing::Ui) -> boing::UniBox {
        let vbox = ui.create_vertical_box().unwrap();
        vbox.append_child(&mut ui.create_spinbox(0, 100).unwrap(), false);

        vbox.append_child(&mut ui.create_slider(0, 100).unwrap(), false);
        vbox.append_child(&mut ui.create_progress_bar().unwrap(), false);

        let mut loading_bar = ui.create_progress_bar().unwrap();
        loading_bar.set_value(-1);
        vbox.append_child(&mut loading_bar, false);

        vbox
    }

    fn create_lists_group(ui: &boing::Ui) -> boing::Group {
        let group = ui.create_group("Lists").unwrap();
        group.set_child(&mut create_lists_vbox(ui));

        group
    }

    fn create_lists_vbox(ui: &boing::Ui) -> boing::UniBox {
        ui.create_vertical_box().unwrap()
    }

    fn create_data_choosers_page(ui: &boing::Ui) -> boing::UniBox {
        let _vbox = ui.create_vertical_box().unwrap();
        let hbox = ui.create_horizontal_box().unwrap();

        hbox
    }
}
