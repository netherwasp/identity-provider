use axum::{
    body::Body,
    http::{HeaderValue, Response, header::SET_COOKIE},
};

pub fn set_cookie(
    response: &mut Response<Body>,
    key: &str,
    value: &str,
    path: &str,
    max_age: usize,
) {
    response.headers_mut().append(
        SET_COOKIE,
        format!("{key}={value}; HttpOnly; SameSite-Lax; Path={path}; Max-Age={max_age}")
            .parse::<HeaderValue>()
            .unwrap(),
    );
}

pub fn generate_nonce(size: usize) -> String {
    let mut nonce_bytes = vec![0u128; size];
    rand::fill(&mut nonce_bytes[..]);
    let nonce = nonce_bytes
        .iter()
        .fold(0u128, |acc, &value| (acc << 8) | value as u128);
    nonce.to_string()
}

pub fn parse_cookie(cookie: &str, key: &str) -> Option<String> {
    cookie.split(";").find_map(|c| {
        let mut parts = c.trim().splitn(2, "=");
        match (parts.next(), parts.next()) {
            (Some(part_key), Some(part_value)) if part_key == key => Some(part_value.to_string()),
            _ => None,
        }
    })
}
