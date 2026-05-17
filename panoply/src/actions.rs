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

pub fn generate_password(
    length: usize,
    use_lowercase: bool,
    use_uppercase: bool,
    use_digits: bool,
    use_special: bool,
) -> String {
    use rand::Rng;

    let mut charset = Vec::new();
    if use_lowercase {
        charset.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz");
    }
    if use_uppercase {
        charset.extend_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_digits {
        charset.extend_from_slice(b"0123456789");
    }
    if use_special {
        charset.extend_from_slice(b"!@#$%^&*");
    }

    if charset.is_empty() {
        return String::new();
    }

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}
