fn main() {
    boing::Ui::run(|ui| {
        let menu = ui.create_menu("File").unwrap();
        menu.append_quit_item().unwrap();

        let window = ui.create_window("XXX", 200, 200, true, true).unwrap();

        let hbox = ui.create_horizontal_box().unwrap();
        let vbox = ui.create_vertical_box().unwrap();
        hbox.set_padded(true);
        vbox.set_padded(true);

        vbox.append_child(ui.create_button("XXX").unwrap(), true);
        hbox.append_child(vbox, true);
        window.set_child(hbox);

        window.show();
    })
    .unwrap();
}
