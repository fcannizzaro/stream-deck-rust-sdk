use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

pub fn image_to_base64(img: Option<Vec<u8>>) -> Option<String> {
    if let Some(img) = img {
        Some(format!(
            "data:image/png;base64,{}",
            general_purpose::STANDARD.encode(img)
        ))
    } else {
        None
    }
}
