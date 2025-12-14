#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use wasm_bindgen::JsValue;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn init() -> Result<(), JsValue> {
    Ok(())
}
