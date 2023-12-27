use super::*;

#[cfg(feature="wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Default)]
/// Builder pattern for Picker struct
pub struct PickerBuilder {
  camera: Option<Camera>,
}

#[cfg(feature="wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl PickerBuilder {
  /// Create an empty `PickerBuilder`
  pub fn default() -> PickerBuilder { std::default::Default::default() }

  /// Specify a `Camera` object
  pub fn camera(mut self, camera: Camera) -> PickerBuilder { self.camera = Some(camera); self }

  /// Builder a `Picker`
  pub fn build(self) -> Result<Picker, JsError> {
    Ok(Picker::from_camera(self.camera.ok_or("Camera is not specified")?))
  }
}



