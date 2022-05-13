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
    window.set_resizeable(false);

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
        menu.append_new_item(ui, "Open").unwrap();
        menu.append_new_item(ui, "Open Folder...")
            .unwrap();
        menu.append_separator();
        menu.append_new_item(ui, "Save").unwrap();
        menu.append_new_item(ui, "Save As...").unwrap();
        menu.append_new_quit_item(ui).unwrap();
    }

    fn setup_edit_menu<'ui>(ui: &'ui boing::Ui) {
        let menu = ui.create_menu("Edit").unwrap();

        let checkable = menu.append_new_item(ui, "Checkable").unwrap();
        checkable.set_checked(true);
        checkable.on_clicked(|item| {
            // Toggle the checkbox.
            item.set_checked(!item.is_checked());
        });

        let disabled = menu.append_new_item(ui, "Disabled").unwrap();
        disabled.disable();

        menu.append_new_preferences_item(ui).unwrap();
    }
    fn setup_help_menu<'ui>(ui: &'ui boing::Ui) {
        let menu = ui.create_menu("Help").unwrap();
        menu.append_new_item(ui, "Documentation").unwrap();
        menu.append_new_about_item(ui).unwrap();
    }
}

mod tab {
    pub fn create(ui: &boing::Ui) -> boing::Tab {
        let tab = ui.create_tab().unwrap();
        let basic_controls = tab.append_new_page("Basic Controls", &mut basic_controls::create(ui))
            .unwrap();
        let numbers_n_lists = tab.append_new_page("Numbers and Lists", &mut numbers_n_lists::create(ui))
            .unwrap();
        let data_choosers = tab.append_new_page("Data Choosers", &mut data_choosers::create(ui))
            .unwrap();

        tab.set_page_margined(basic_controls, true);
        tab.set_page_margined(numbers_n_lists, true);
        tab.set_page_margined(data_choosers, true);

        tab
    }

    mod basic_controls {
        use std::ops::DerefMut;

        pub fn create(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            static LABEL_TEXT: &str = "This is a label. Right now, labels can only span one line.";

            let axis = ui.create_vertical_axis().unwrap();
            axis.set_padded(true);
            axis.append_new_child(&mut create_x_axis(ui), false);
            axis.append_new_child(&mut ui.create_label(LABEL_TEXT).unwrap(), false);
            axis.append_new_child(&mut ui.create_horizontal_separator().unwrap(), false);

            axis
        }

        fn create_x_axis(ui: &boing::Ui) -> boing::Axis {
            let axis = ui.create_horizontal_axis().unwrap();
            axis.set_padded(true);
            axis.append_new_child(&mut ui.create_pushbutton("Button").unwrap(), false);
            axis.append_new_child(&mut ui.create_checkbox("Checkbox").unwrap(), false);

            axis
        }
    }

    mod numbers_n_lists {
        use std::ops::DerefMut;

        pub fn create(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            let axis = ui.create_horizontal_axis().unwrap();
            axis.append_new_child(&mut create_numbers_group(ui), true);
            axis.append_new_child(&mut create_lists_group(ui), true);

            axis
        }

        fn create_numbers_group(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            let group = ui.create_group("Numbers").unwrap();
            group.set_margined(true);
            group.set_child(&mut create_numbers_axis(ui));

            group
        }

        fn create_numbers_axis(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            let axis = ui.create_vertical_axis().unwrap();
            axis.append_new_child(&mut ui.create_spinbox(0, 100).unwrap(), false);
            axis.append_new_child(&mut ui.create_slider(0, 100).unwrap(), false);
            axis.append_new_child(&mut ui.create_progress_bar().unwrap(), false);
            axis.append_new_child(&mut create_loading_bar(ui), false);

            axis
        }

        fn create_loading_bar(ui: &boing::Ui) -> boing::ProgressBar {
            let loading_bar = ui.create_progress_bar().unwrap();
            loading_bar.set_indefinite();

            loading_bar
        }

        fn create_lists_group(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            let group = ui.create_group("Lists").unwrap();
            group.set_margined(true);
            group.set_child(&mut create_lists_axis(ui));

            group
        }

        fn create_lists_axis(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            let axis = ui.create_vertical_axis().unwrap();
            axis.append_new_child(&mut create_radio_buttons(ui), false);

            axis
        }

        fn create_radio_buttons(ui: &boing::Ui) -> boing::RadioButtons {
            let mut buttons = ui.create_radio_buttons().unwrap();
            buttons.append_new_item("Radio Button 1").unwrap();
            buttons.append_new_item("Radio Button 2").unwrap();
            buttons.append_new_item("Radio Button 3").unwrap();
            buttons.append_new_item("Radio Button 4").unwrap();

            buttons
        }
    }

    mod data_choosers {
        use std::ops::DerefMut;

        pub fn create(ui: &boing::Ui) -> impl DerefMut<Target = boing::Control> {
            let axis = ui.create_horizontal_axis().unwrap();
            axis.set_padded(true);
            axis.append_new_child(&mut create_picker_axis(ui), false);
            axis.append_new_child(&mut ui.create_vertical_separator().unwrap(), false);

            axis
        }

        fn create_picker_axis(ui: &boing::Ui) -> boing::Axis {
            let axis = ui.create_vertical_axis().unwrap();
            axis.set_padded(true);
            axis.append_new_child(&mut ui.create_font_picker().unwrap(), false);
            axis.append_new_child(&mut ui.create_color_picker().unwrap(), false);

            axis
        }
    }
}
