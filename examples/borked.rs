fn main() {
    boing::Ui::run(|ui| {
        let mut menu = ui.create_menu("File").unwrap();
        menu.append_quit_item().unwrap();

        let mut window = ui.create_window("XXX", 200, 200, true, true).unwrap();

        let mut bibox = ui.create_biaxial_box(true).unwrap();
        bibox.set_padded(true);

        let button = ui.create_button("XXX").unwrap();
        bibox.append_child(ui, button);

        window.set_child(ui, bibox);
        window.show();
    })
    .unwrap();
}
