fn main() {
    boing::Ui::run(|ui| {
        let mut menu = ui.create_menu("File").unwrap();

        let mut window = ui.create_window("XXX", 200, 200, false).unwrap();

        let button = ui.create_button("XXX").unwrap();
        window.set_child(ui, button);

        window.show();
    })
    .unwrap();
}
