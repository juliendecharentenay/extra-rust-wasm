use super::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(derive_builder::Builder)]
#[builder(build_fn(error = "Error"))]
/// Store a camera when processing a mouse down, move and up events.
pub struct MouseCamera {
  camera: Camera,
  mouse_down: web_sys::MouseEvent,
  mouse_move: web_sys::MouseEvent,
  #[builder(default = "false")]
  panning: bool,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl MouseCamera {
  // Retrieve current camera
  fn camera(&self) -> Camera { 
    self.camera.orbit(
      self.mouse_down.client_x() as f32,
      self.mouse_down.client_y() as f32,
      self.mouse_move.client_x() as f32,
      self.mouse_move.client_y() as f32,
    ) 
  }

  /// Retrieve the underlying `Camera`
  pub fn to_camera(self) -> Result<Camera, JsError> { Ok(self.camera()) }

  /// Retrieve the underlying `Camera`
  pub fn as_camera(&self) -> Result<Camera, JsError> { Ok(self.camera()) }

  /// Convert the camera to a 4x4 view-projection matrix
  pub fn as_matrix(&self) -> Result<Vec<f32>, JsError> { self.camera().as_matrix() }

  /// Convert the camera to a 4x4 view matrix
  pub fn as_view_matrix(&self) -> Result<Vec<f32>, JsError> { self.camera().as_view_matrix() }

  /// Convert the camera to a 4x4 view matrix
  pub fn as_projection_matrix(&self) -> Result<Vec<f32>, JsError> { self.camera().as_projection_matrix() }

  /// Handle mouse move event
  pub fn on_mouse_move(mut self, event: web_sys::MouseEvent) -> Result<MouseCamera, JsError> {
    self.mouse_move = event;
    self.panning = true;
    Ok(self)
  }

  /// Handle mouse up event
  pub fn on_mouse_up(self, event: web_sys::MouseEvent) -> Result<Camera, JsError> {
    let camera = if self.panning {
      self.on_mouse_move(event)?.camera()
    } else {
      self.camera().with_mouse_select(event)
    };
    Ok(camera)
  }

  /// Handle `wheel` event
  pub fn on_wheel(self, event: web_sys::WheelEvent) -> Result<Camera, JsError> {
    let e = self.mouse_move.clone();
    self.on_mouse_up(e)
    .and_then(|c| c.on_wheel(event))
  }

  /// Retrieve the udpate status
  pub fn updated(&self) -> bool { true }

  /// Trigger a pick hover
  pub fn pick_hover(&self) -> Result<wasm_bindgen::JsValue, JsError> { Ok(wasm_bindgen::JsValue::NULL) }

  /// Trigger a pick select
  pub fn pick_select(&self) -> Result<wasm_bindgen::JsValue, JsError> { Ok(wasm_bindgen::JsValue::NULL) }
}

