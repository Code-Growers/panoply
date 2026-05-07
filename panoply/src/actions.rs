pub fn base64_encode(input: &str) -> Result<String, String> {
    use base64::{Engine as _, engine::general_purpose};
    Ok(general_purpose::STANDARD.encode(input.as_bytes()))
}

pub fn base64_decode(input: &str) -> Result<String, String> {
    use base64::{Engine as _, engine::general_purpose};
    let bytes = general_purpose::STANDARD
        .decode(input)
        .map_err(|e| e.to_string())?;
    String::from_utf8(bytes).map_err(|e| e.to_string())
}

pub fn url_encode(input: &str) -> Result<String, String> {
    Ok(urlencoding::encode(input).to_string())
}

pub fn url_decode(input: &str) -> Result<String, String> {
    urlencoding::decode(input)
        .map(|s| s.to_string())
        .map_err(|e| e.to_string())
}

pub fn generate_password(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
