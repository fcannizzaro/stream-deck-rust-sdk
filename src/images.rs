pub fn image_to_base64(img: Vec<u8>) -> String {
    format!("data:image/png;base64,{}", base64::encode(img))
}
