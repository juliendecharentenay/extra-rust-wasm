use super::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Default)]
pub struct HexahedronBuilder {
  start: Option<nalgebra::Point3<f32>>,
  end: Option<nalgebra::Point3<f32>>,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl HexahedronBuilder {
  /// Create an empty `HexahedronBuilder`
  pub fn new() -> HexahedronBuilder { HexahedronBuilder::default() }

  /// Specify start point
  pub fn start(mut self, start: &[f32]) -> HexahedronBuilder { self.start = Some(nalgebra::Point3::from_slice(start)); self }

  /// Specify end point
  pub fn end(mut self, end: &[f32]) -> HexahedronBuilder { self.end = Some(nalgebra::Point3::from_slice(end)); self }

  /// Build an `Hexahedron` object
  pub fn build(self) -> Result<Hexahedron, JsError> {
    Ok(
      Hexahedron::new(
        self.start.ok_or("Start is not specified")?,
        self.end.ok_or("End is not specified")?,
      )?
    )
  }
}

