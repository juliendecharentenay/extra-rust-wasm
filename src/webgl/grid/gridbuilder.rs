use super::*;

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Default)]
pub struct GridBuilder {
  normal: Option<nalgebra::Vector3<f32>>,
  tangent: Option<nalgebra::Vector3<f32>>,
  center: Option<nalgebra::Point3<f32>>,
  delta: Option<f32>,
  n: Option<u32>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl GridBuilder {
  /// Create an empty `GridBuilder`
  pub fn new() -> GridBuilder {
    GridBuilder {
      normal: None,
      tangent: None,
      center: None,
      delta: None,
      n: None,
    }
  }

  /// Specify normal vector
  pub fn normal(mut self, normal: &[f32]) -> GridBuilder { self.normal = Some(nalgebra::Vector3::from_row_slice(normal)); self }

  /// Specify tangent vector
  pub fn tangent(mut self, tangent: &[f32]) -> GridBuilder { self.tangent = Some(nalgebra::Vector3::from_row_slice(tangent)); self }

  /// Specify center point
  pub fn center(mut self, center: &[f32]) -> GridBuilder { self.center = Some(nalgebra::Point3::from_slice(center)); self }

  /// Specify grid spacing
  pub fn delta(mut self, delta: f32) -> GridBuilder { self.delta = Some(delta); self }

  /// Specify number of grid cells
  pub fn n(mut self, n: u32) -> GridBuilder { self.n = Some(n); self }

  /// Build a `Grid` object
  pub fn build(self) -> Result<Grid, JsError> {
    Ok(
      Grid::new(
        self.normal.ok_or("Normal not specified")?,
        self.tangent.ok_or("Tangent not specified")?,
        self.center.ok_or("Center not specified")?,
        self.delta.ok_or("Grid spacing (delta) not specified")?,
        self.n.ok_or("Number of grid cell (n) not specified")?,
      )?
    )
  }
}

