
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Translation { 
  x: f32,
  y: f32,
  z: f32,
}

impl Translation {
  pub const NAME: &str = "Translation";
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Translation {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Translation { x, y, z }
  }

  /// Retrieve the transformation name
  pub fn name(&self) -> String { Translation::NAME.to_string() }

  pub fn combine(&self, other: Translation) -> Self {
    Translation { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
  }

  pub fn clone(&self) -> Self {
    Clone::clone(self)
  }
}

impl Translation {
  pub fn matrix4(&self) -> nalgebra::Matrix4<f32> {
    nalgebra::geometry::Translation3::new(self.x, self.y, self.z).to_homogeneous()
  }
}
