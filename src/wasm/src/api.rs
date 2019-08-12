use wasm_bindgen::prelude::*;

// test
#[wasm_bindgen]
pub fn test() -> Result<f64, JsValue> {

  Ok( 42. )
}
