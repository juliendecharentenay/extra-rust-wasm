
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Identity { }

#[wasm_bindgen::prelude::wasm_bindgen]
impl Identity {
  pub fn new() -> Self { Self::default() }
}


impl Identity {
  pub fn matrix4(&self) -> nalgebra::Matrix4<f32> {
    nalgebra::Matrix4::identity()
  }
}
