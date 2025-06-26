use iced::window;
use iced::window::icon::from_rgba;
use image::io::Reader as ImageReader;
use std::io::Cursor;

pub fn load_icon() -> window::Icon {
    let img_bytes = include_bytes!("../rust.ico");
    let img = ImageReader::new(Cursor::new(img_bytes))
        .with_guessed_format()
        .expect("Error al leer la imagen")
        .decode()
        .expect("Error al decodificar la imagen")
        .into_rgba8();

    let (width, height) = img.dimensions();
    let rgba = img.into_raw();

    from_rgba(rgba, width, height).unwrap()
}
