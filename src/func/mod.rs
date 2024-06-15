use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;
use std::env;

pub fn get_arg(idx: usize) -> String {
    env::args().nth(idx).unwrap_or("".to_string())
}

#[derive(Serialize)]
struct Claims {
    iss: String,
    iat: u64,
}

pub fn get_ios_activity_push_token(private_key: &str) -> String {
    let mut header = Header::new(Algorithm::ES256);
    header.kid = Some("G6BRSC9PWX".to_string());
    header.typ = None;

    let claims = Claims {
        iss: "YVKZAX7JYL".to_string(),
        iat: chrono::Local::now().timestamp() as u64,
    };

    let encoding_key = EncodingKey::from_ec_pem(private_key.as_bytes())
        .expect("Failed to create encoding key from PEM");

    match encode(&header, &claims, &encoding_key) {
        Ok(token) => token,
        Err(err) => {
            println!("Failed to encode token: {}", err);
            "".to_string()
        }
    }
}
