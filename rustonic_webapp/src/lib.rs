#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use rustonic_shared::*;
use wasm_bindgen::JsValue;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn init() -> Result<(), JsValue> {
    let ui = AppWindow::new().map_err(|e| JsValue::from_str(&e.to_string()))?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_counter(ui.get_counter() + 1);
            }
        }
    });

    ui.run().map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}
