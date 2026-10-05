use wasm_bindgen::prelude::*;
fn js_error(e: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&e.to_string())
}
#[wasm_bindgen]
pub fn registry_json() -> Result<String, JsValue> {
    serde_json::to_string(&drawlab_core::profiles().map_err(js_error)?).map_err(js_error)
}
#[wasm_bindgen]
pub fn generate_json(profile: &str, mode: &str, count: u32) -> Result<String, JsValue> {
    let mode = match mode {
        "selection" => drawlab_core::Mode::Selection,
        "draw" => drawlab_core::Mode::Draw,
        _ => return Err(js_error("INVALID_MODE")),
    };
    serde_json::to_string_pretty(
        &drawlab_core::generate(profile, mode, count as usize).map_err(js_error)?,
    )
    .map_err(js_error)
}
#[wasm_bindgen]
pub fn verify_json(json: &str) -> Result<String, JsValue> {
    let r = drawlab_core::verify(json).map_err(js_error)?;
    Ok(format!(
        "Valid: {} board(s), {}. This checks mathematical validity, not authenticity or randomness.",
        r.boards.len(),
        r.profile.id
    ))
}
#[wasm_bindgen]
pub fn odds_json(id: &str) -> Result<String, JsValue> {
    let p = drawlab_core::profiles()
        .map_err(js_error)?
        .into_iter()
        .find(|p| p.id == id)
        .ok_or_else(|| js_error("UNKNOWN_PROFILE"))?;
    serde_json::to_string(&drawlab_core::odds(&p).map_err(js_error)?).map_err(js_error)
}
#[wasm_bindgen]
pub fn version() -> String {
    drawlab_core::VERSION.into()
}
