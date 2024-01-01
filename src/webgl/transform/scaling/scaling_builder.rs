use super::*;

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Default)]
pub struct ScalingBuilder {
  origin: Option<nalgebra::Point3<f32>>,
  vector: Option<nalgebra::Vector3<f32>>,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl ScalingBuilder {
  /// Expose default
  pub fn default() -> ScalingBuilder {
    std::default::Default::default()
  }

  /// Set origin
  pub fn with_origin(mut self, x: f32, y: f32, z: f32) -> Self {
    self.origin = Some(nalgebra::Point3::new(x, y, z));
    self
  }

  /// Set scaling vector (direction and magnitude)
  pub fn with_vector(mut self, x: f32, y: f32, z: f32) -> Self {
    self.vector = Some(nalgebra::Vector3::new(x, y, z));
    self
  }

  /// Build scaling transformation
  pub fn build(self) -> Result<Scaling, JsError> {
    Ok(Scaling::from_origin_vector(
      self.origin.ok_or(wasm_bindgen::JsValue::from_str("Unable to build Scaling transform: origin is not specified"))?,
      self.vector.ok_or(wasm_bindgen::JsValue::from_str("Unable to build Scaling transform: vector is not specified"))?,
    ))
  }
}
