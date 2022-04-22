fn main() {
    let mut ui = boing::Ui::new().unwrap();

    let mut window = ui.create_window(
        "Hello World".into(),
        500,
        500,
        true,
    )
    .unwrap();

    window.set_borderless(false);
    window.set_fullscreen(false);
    window.set_resizeable(true);
    window.enable();
    window.show();

    ui.start();
}
