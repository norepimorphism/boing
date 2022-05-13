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
        tab.append_page("Basic Controls", &mut basic_controls::create(ui))
            .unwrap();
        tab.append_page("Numbers and Lists", &mut numbers_n_lists::create(ui))
            .unwrap();
        tab.append_page("Data Choosers", &mut data_choosers::create(ui))
            .unwrap();

        tab
    }

    mod basic_controls {
        use std::ops::DerefMut;

        pub fn create(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            static LABEL_TEXT: &str = "This is a label. Right now, labels can only span one line.";

            let y_axis = ui.create_vertical_axis().unwrap();
            y_axis.set_padded(true);

            let mut button_axis = ui.create_horizontal_axis().unwrap();
            button_axis.set_padded(true);
            button_axis.append_child(&mut ui.create_pushbutton("Button").unwrap(), true);

            y_axis.append_child(&mut button_axis, false);
            y_axis.append_child(&mut ui.create_label(LABEL_TEXT).unwrap(), false);

            y_axis
        }
    }

    mod numbers_n_lists {
        use std::ops::DerefMut;

        pub fn create(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            let x_axis = ui.create_horizontal_axis().unwrap();
            x_axis.append_child(&mut create_numbers_group(ui), true);
            x_axis.append_child(&mut create_lists_group(ui), true);

            x_axis
        }

        fn create_numbers_group(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            let group = ui.create_group("Numbers").unwrap();
            group.set_child(&mut create_numbers_axis(ui));

            group
        }

        fn create_numbers_axis(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            let y_axis = ui.create_vertical_axis().unwrap();
            y_axis.append_child(&mut ui.create_spinbox(0, 100).unwrap(), false);
            y_axis.append_child(&mut ui.create_slider(0, 100).unwrap(), false);
            y_axis.append_child(&mut ui.create_progress_bar().unwrap(), false);

            let mut loading_bar = ui.create_progress_bar().unwrap();
            loading_bar.set_value(-1);
            y_axis.append_child(&mut loading_bar, false);

            y_axis
        }

        fn create_lists_group(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            let group = ui.create_group("Lists").unwrap();
            group.set_child(&mut create_lists_axis(ui));

            group
        }

        fn create_lists_axis(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            ui.create_vertical_axis().unwrap()
        }
    }

    mod data_choosers {
        use std::ops::DerefMut;

        pub fn create(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            let _y_axis = ui.create_vertical_axis().unwrap();
            let x_axis = ui.create_horizontal_axis().unwrap();

            x_axis
        }
    }
}
