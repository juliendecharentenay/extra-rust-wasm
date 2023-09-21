
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Default)]
pub struct Translation { 
  x: f32,
  y: f32,
  z: f32,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Translation {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Translation { x, y, z }
  }
}

impl Translation {
  pub fn matrix4(&self) -> nalgebra::Matrix4<f32> {
    nalgebra::geometry::Translation3::new(self.x, self.y, self.z).to_homogeneous()
  }
}
