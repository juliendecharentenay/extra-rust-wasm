use super::*;

mod scaling_builder; pub use scaling_builder::ScalingBuilder;

#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Scaling {
  matrix: nalgebra::Matrix4<f32>,
}

impl Scaling {
  pub const NAME: &str = "Scaling";

  fn from_origin_vector(origin: nalgebra::Point3<f32>, vector: nalgebra::Vector3<f32>) -> Self {
    let matrix = 
      nalgebra::geometry::Translation3::new(origin.x, origin.y, origin.z).to_homogeneous()
      * nalgebra::geometry::Scale3::new(vector.x, vector.y, vector.z).to_homogeneous()
      * nalgebra::geometry::Translation3::new(-origin.x, -origin.y, -origin.z).to_homogeneous();
    Scaling { matrix, }
  }
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Scaling {
  /// Retrieve the transformation name
  pub fn name(&self) -> String { Scaling::NAME.to_string() }

  /// Combine with another scaling
  pub fn combine(&self, other: Scaling) -> Self {
    Scaling {
      matrix: self.matrix * other.matrix,
    }
  }

  pub fn clone(&self) -> Self {
    Clone::clone(self)
  }
}

impl Scaling {
  pub fn matrix4(&self) -> nalgebra::Matrix4<f32> {
    self.matrix.clone()
  }
}

