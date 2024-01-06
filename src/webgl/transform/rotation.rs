use super::*;

mod rotation_builder; pub use rotation_builder::RotationBuilder;

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Rotation {
  matrix: nalgebra::Matrix4<f32>,
}

impl Rotation {
  pub const NAME: &str = "Rotation";

  fn from_origin_vector(origin: nalgebra::Point3<f32>, vector: nalgebra::Vector3<f32>) -> Self {
    let matrix = 
      nalgebra::geometry::Translation3::new(origin.x, origin.y, origin.z).to_homogeneous()
      * nalgebra::geometry::Rotation3::from_scaled_axis(vector).to_homogeneous()
      * nalgebra::geometry::Translation3::new(-origin.x, -origin.y, -origin.z).to_homogeneous();
    Rotation { matrix, }
  }
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Rotation {
  /// Retrieve the transformation name
  pub fn name(&self) -> String { Rotation::NAME.to_string() }

  /// Combine with another rotation
  pub fn combine(&self, other: Rotation) -> Self {
    Rotation {
      matrix: self.matrix * other.matrix,
    }
  }

  /// Expose clone functionality to javascript
  pub fn clone(&self) -> Self {
    Clone::clone(self)
  }
}

impl Rotation {
  pub fn matrix4(&self) -> nalgebra::Matrix4<f32> {
    self.matrix.clone()
  }
}


