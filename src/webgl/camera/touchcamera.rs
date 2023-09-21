use super::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(derive_builder::Builder)]
#[builder(build_fn(error = "Error"))]
/// Store a camera processing touch events
pub struct TouchCamera {
  camera: Camera,
  #[builder(default)]
  touches: std::collections::HashMap<i32, web_sys::Touch>,
  #[builder(default)]
  touches_down: std::collections::HashMap<i32, web_sys::Touch>,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl TouchCamera {
  // Retrieve current camera
  fn camera(&self) -> Camera {
    let (fr_x, fr_y) = TouchCamera::touch_mid(&self.touches_down);
    let fr_l         = TouchCamera::touch_delta(&self.touches_down, &(fr_x, fr_y));

    let (to_x, to_y) = TouchCamera::touch_mid(&self.touches);
    let to_l         = TouchCamera::touch_delta(&self.touches, &(to_x, to_y));
    let alpha        = TouchCamera::touch_alpha(&self.touches_down, &self.touches);

    self.camera
    .orbit(fr_x, fr_y, to_x, to_y)
    .modify_view(nalgebra::Matrix4::from_euler_angles(0f32, 0f32, -alpha))
    .zoom(to_x, to_y, to_l - fr_l)
  }

  /// Retrieve the underlying `Camera`
  pub fn to_camera(self) -> Result<Camera, JsError> { Ok(self.camera()) }

  /// Retrieve the underlying `Camera`
  pub fn as_camera(&self) -> Result<Camera, JsError> { Ok(self.camera()) }

  /// Convert the camera to a 4x4 view-projection matrix
  pub fn as_matrix(&self) -> Result<Vec<f32>, JsError> { self.camera().as_matrix() }

  /// Convert the camera to a 4x4 view matrix
  pub fn as_view_matrix(&self) -> Result<Vec<f32>, JsError> { self.camera().as_view_matrix() }

  /// Convert the camera to a 4x4 projection matrix
  pub fn as_projection_matrix(&self) -> Result<Vec<f32>, JsError> { self.camera().as_projection_matrix() }

  /// Handle mouse down event
  pub fn on_mouse_down(self, event: web_sys::MouseEvent) -> Result<MouseCamera, JsError> {
    self.camera().on_mouse_down(event)
  }

  /// Handle mouse move event
  pub fn on_mouse_move(self, _event: web_sys::MouseEvent) -> Result<TouchCamera, JsError> {
    Ok(self)
  }

  /// Handle mouse up event
  pub fn on_mouse_up(self, _event: web_sys::MouseEvent) -> Result<TouchCamera, JsError> {
    Ok(self)
  }

  /// Handle wheel event
  pub fn on_wheel(self, event: web_sys::WheelEvent) -> Result<Camera, JsError> {
    self.camera().on_wheel(event)
  }

  /// Handle touch events
  pub fn on_touch(mut self, event: web_sys::TouchEvent) -> Result<TouchCamera, JsError> {
    event.prevent_default();
    let touch_list = event.changed_touches();
    match event.type_().as_str() {
      "touchstart" => {
        if ! self.touches_down.is_empty() { self.camera = self.camera(); }
        for i in 0..touch_list.length() {
          if let Some(touch) = touch_list.get(i) {
            self.touches.insert(touch.identifier(), touch);
          }
        }
        self.touches_down = self.touches.clone();
        Ok(self)
      },

      "touchmove" => {
        for i in 0..touch_list.length() {
          if let Some(touch) = touch_list.get(i) {
            self.touches.insert(touch.identifier(), touch);
          }
        }
        Ok(self)
      },

      "touchend" | "touchcancel" => {
        if ! self.touches_down.is_empty() { self.camera = self.camera(); }
        for i in 0..touch_list.length() {
          if let Some(touch) = touch_list.get(i) {
            self.touches.remove(&touch.identifier());
          }
        }
        self.touches_down = self.touches.clone();
        Ok(self)
      },

      _ => Err(format!("Event type {} is not supported", event.type_()).into()),
    }
  }
}

#[cfg(feature = "wasm")]
impl TouchCamera {
  fn touch_mid(touches: &std::collections::HashMap<i32, web_sys::Touch>) -> (f32, f32) {
    let alpha: f32 = 1f32 / if touches.len() > 0 { touches.len() as f32 } else { 1f32 };
    touches.iter().fold((0f32, 0f32), |r, (_, t)| (r.0 + alpha*t.client_x() as f32, r.1 + alpha*t.client_y() as f32))
  }

  fn touch_delta(touches: &std::collections::HashMap<i32, web_sys::Touch>, mid: &(f32, f32)) -> f32 {
    touches.iter()
    .fold(0f32, |r, (_, t)| { r + ((t.client_x() as f32 - mid.0).powi(2) + (t.client_y() as f32 - mid.1).powi(2)).sqrt() })
  }

  fn alpha(a: &web_sys::Touch, b: &web_sys::Touch) -> f32 {
    let l = ((b.client_x() as f32 - a.client_x() as f32).powi(2) + (b.client_y() as f32 - a.client_y() as f32).powi(2)).sqrt();
    if l > 1e-5 {
      if b.client_y() > a.client_y() {
        1f32 * ((b.client_x() as f32 - a.client_x() as f32)/l).acos()
      } else {
        -1f32 * ((b.client_x() as f32 - a.client_x() as f32)/l).acos()
      }
    } else {
      0f32
    }
  }

  fn touch_alpha(touches_from: &std::collections::HashMap<i32, web_sys::Touch>, touches_to: &std::collections::HashMap<i32, web_sys::Touch>) -> f32 {
    let mut reference: Option<(&web_sys::Touch, &web_sys::Touch)> = None;
    let mut alpha = 0f32; let mut count = 0;
    for (k, v_from) in touches_from.iter() {
      if let Some(v_to) = touches_to.get(k) {
        if reference.is_none() { 
          reference = Some((v_from, v_to)); 
        } else {
          count += 1;
          alpha += TouchCamera::alpha(reference.unwrap().1, v_to) - TouchCamera::alpha(reference.unwrap().0, v_from);
        }
      }
    }
    alpha / if count > 0 { count as f32 } else { 1f32 }
  }
}

