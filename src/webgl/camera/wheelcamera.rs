use super::*;

#[derive(derive_builder::Builder)]
#[builder(build_fn(error = "Error"))]
/// Handle a camera to apply wheel events
pub struct WheelCamera {
  camera: Camera,
}

impl WheelCamera {
  pub fn on_wheel(self, event: web_sys::WheelEvent) -> Result<Camera, JsError> {
    Ok(self.camera.zoom(
        event.client_x() as f32,
        event.client_y() as f32,
        event.delta_y() as f32,
      ))
  }

  /// Retrieve the udpate status
  pub fn updated(&self) -> bool { true }

  /// Trigger a pick hover
  pub fn pick_hover(&self) -> Result<wasm_bindgen::JsValue, JsError> { Ok(wasm_bindgen::JsValue::NULL) }

  /// Trigger a pick select
  pub fn pick_select(&self) -> Result<wasm_bindgen::JsValue, JsError> { Ok(wasm_bindgen::JsValue::NULL) }
}

