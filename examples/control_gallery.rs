//! A clone of the libui example program "libui Control Gallery".

fn main() {
    let mut ui = boing::Ui::new().unwrap();

    let mut window = ui.create_window(
        "boing Control Gallery".into(),
        640,
        480,
        true,
    )
    .unwrap();

    window.set_margined(true);
    window.show();

    ui.start();
}
