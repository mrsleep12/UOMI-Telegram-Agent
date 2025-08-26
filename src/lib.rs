use wasm_bindgen::prelude::*;

/// Return redirect URL ke akun Telegram kamu
#[wasm_bindgen]
pub fn get_redirect_url() -> String {
    "https://t.me/Mrsleep1".to_string()
}
