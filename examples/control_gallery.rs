// SPDX-License-Identifier: MPL-2.0

//! A clone of the libui example program "libui Control Gallery".

// #![windows_subsystem = "windows"]

fn main() {
    setup_tracing();

    let ui = boing::Ui::new().unwrap();
    menubar::setup(&ui);

    let window = ui
        .create_window("libui Control Gallery", 240, 240, true, true)
        .unwrap();
    window.set_margined(true);
    window.set_resizeable(true);

    let tab = tab::create(&ui);
    window.set_child(tab);
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
        menu.push_new_item("Open").unwrap();
        menu.push_new_item("Open Folder...")
            .unwrap();
        menu.push_separator();
        menu.push_new_item("Save").unwrap();
        menu.push_new_item("Save As...").unwrap();
        menu.push_new_quit_item().unwrap();
    }

    fn setup_edit_menu<'ui>(ui: &'ui boing::Ui) {
        let menu = ui.create_menu("Edit").unwrap();

        let checkable = menu.push_new_item("Checkable").unwrap();
        checkable.set_checked(true);
        checkable.on_clicked(|item| {
            // Toggle the checkbox.
            item.set_checked(!item.is_checked());
        });

        let disabled = menu.push_new_item("Disabled").unwrap();
        disabled.disable();

        menu.push_new_preferences_item().unwrap();
    }

    fn setup_help_menu<'ui>(ui: &'ui boing::Ui) {
        let menu = ui.create_menu("Help").unwrap();
        menu.push_new_item("Documentation").unwrap();

        let about = menu.push_new_about_item().unwrap();
        about.on_clicked(|_| {
            let window = ui.create_window("About libui Control Gallery", 256, 256, false, false).unwrap();
            window.show();
        })
    }
}

mod tab {
    pub fn create(ui: &boing::Ui) -> &mut boing::Tab {
        let tab = ui.create_tab().unwrap();
        let basic_controls = tab.push_new_page("Basic Controls", basic_controls::create(ui))
            .unwrap();
        let numbers_n_lists = tab.push_new_page("Numbers and Lists", numbers_n_lists::create(ui))
            .unwrap();
        let data_choosers = tab.push_new_page("Data Choosers", data_choosers::create(ui))
            .unwrap();

        tab.set_page_margined(basic_controls, true);
        tab.set_page_margined(numbers_n_lists, true);
        tab.set_page_margined(data_choosers, true);

        tab
    }

    mod basic_controls {
        pub fn create(ui: &boing::Ui) -> &mut boing::Axis {
            static LABEL_TEXT: &str = "This is a label. Right now, labels can only span one line.";

            let axis = ui.create_vertical_axis().unwrap();
            axis.set_padded(true);
            axis.push_new_child(create_x_axis(ui), false);
            axis.push_new_child(ui.create_label(LABEL_TEXT).unwrap(), false);
            axis.push_new_child(ui.create_horizontal_separator().unwrap(), false);

            axis
        }

        fn create_x_axis(ui: &boing::Ui) -> &mut boing::Axis {
            let axis = ui.create_horizontal_axis().unwrap();
            axis.set_padded(true);
            axis.push_new_child(ui.create_pushbutton("Button").unwrap(), false);
            axis.push_new_child(ui.create_checkbox("Checkbox").unwrap(), false);

            axis
        }
    }

    mod numbers_n_lists {
        pub fn create(ui: &boing::Ui) -> &mut boing::Axis {
            let axis = ui.create_horizontal_axis().unwrap();
            axis.push_new_child(create_numbers_group(ui), true);
            axis.push_new_child(create_lists_group(ui), true);

            axis
        }

        fn create_numbers_group(ui: &boing::Ui) -> &mut boing::Group {
            let group = ui.create_group("Numbers").unwrap();
            group.set_margined(true);
            group.set_child(create_numbers_axis(ui));

            group
        }

        fn create_numbers_axis(ui: &boing::Ui) -> &mut boing::Axis {
            let axis = ui.create_vertical_axis().unwrap();
            axis.push_new_child(ui.create_spinbox(0, 100).unwrap(), false);

            let slider = ui.create_slider(0, 100).unwrap();
            let progress_bar = ui.create_progress_bar().unwrap();
            axis.push_new_child(slider, false);
            axis.push_new_child(progress_bar, false);
            slider.on_changed(move |slider| {
                progress_bar.set_value(slider.value());
            });

            axis.push_new_child(create_loading_bar(ui), false);

            axis
        }

        fn create_loading_bar(ui: &boing::Ui) -> &mut boing::ProgressBar {
            let loading_bar = ui.create_progress_bar().unwrap();
            loading_bar.set_as_indefinite();

            loading_bar
        }

        fn create_lists_group(ui: &boing::Ui) -> &mut boing::Group {
            let group = ui.create_group("Lists").unwrap();
            group.set_margined(true);
            group.set_child(create_lists_axis(ui));

            group
        }

        fn create_lists_axis(ui: &boing::Ui) -> &mut boing::Axis {
            let axis = ui.create_vertical_axis().unwrap();
            axis.push_new_child(create_combobox(ui), false);
            axis.push_new_child(create_radio_buttons(ui), false);

            axis
        }

        fn create_combobox(ui: &boing::Ui) -> &mut boing::Combobox {
            let combobox = ui.create_combobox().unwrap();
            combobox.push_new_item("Combobox Item 1").unwrap();
            combobox.push_new_item("Combobox Item 2").unwrap();
            combobox.push_new_item("Combobox Item 3").unwrap();
            combobox.push_new_item("Combobox Item 4").unwrap();

            combobox
        }

        fn create_radio_buttons(ui: &boing::Ui) -> &mut boing::RadioButtons {
            let buttons = ui.create_radio_buttons().unwrap();
            buttons.push_new_item("Radio Button 1").unwrap();
            buttons.push_new_item("Radio Button 2").unwrap();
            buttons.push_new_item("Radio Button 3").unwrap();
            buttons.push_new_item("Radio Button 4").unwrap();

            buttons
        }
    }

    mod data_choosers {
        pub fn create(ui: &boing::Ui) -> &mut boing::Axis {
            let axis = ui.create_horizontal_axis().unwrap();
            axis.set_padded(true);
            axis.push_new_child(create_picker_axis(ui), false);
            axis.push_new_child(ui.create_vertical_separator().unwrap(), false);

            axis
        }

        fn create_picker_axis(ui: &boing::Ui) -> &mut boing::Axis {
            let axis = ui.create_vertical_axis().unwrap();
            axis.set_padded(true);
            axis.push_new_child(ui.create_font_picker().unwrap(), false);
            axis.push_new_child(ui.create_color_picker().unwrap(), false);

            axis
        }
    }
}
