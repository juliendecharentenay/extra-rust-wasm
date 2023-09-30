use super::*;

/// Build a `Camera` object using Builder pattern
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Default)]
pub struct CameraBuilder {
  width:  Option<f32>,
  height: Option<f32>,
  fov:    Option<f32>,
  eye:    Option<nalgebra::Point3<f32>>,
  target: Option<nalgebra::Point3<f32>>,
  up:     Option<nalgebra::Vector3<f32>>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl CameraBuilder {
  /// Construct a new `CameraBuilder`
  pub fn empty() -> CameraBuilder { CameraBuilder::default() }

  /// Convenience function to construct a very basis camera
  pub fn basic() -> CameraBuilder {
    CameraBuilder::default()
    .width(1280f32)
    .height(720f32)
    .fov(45.0*std::f32::consts::PI/180f32)
    .eye(nalgebra::Point3::new(0.0, 5.0, 0.0).coords.as_slice())
    .target(nalgebra::Point3::new(0.0, 0.0, 0.0).coords.as_slice())
    .up(nalgebra::Vector3::new(0.0, 0.0, 1.0).as_slice())
  }

  /// Specify the `width` in pixel of the canvas used
  pub fn width(mut self, width: f32)      -> CameraBuilder { self.width = Some(width); self }
  /// Specify the `height` in pixel of the canvas used
  pub fn height(mut self, height: f32)    -> CameraBuilder { self.height = Some(height); self }
  /// Specify the field of view to be used
  pub fn fov(mut self, fov: f32)          -> CameraBuilder { self.fov = Some(fov); self }
  /// Specify the eye position as a slice `[x, y, z]`
  pub fn eye(mut self, eye: &[f32])       -> CameraBuilder { self.eye = Some(nalgebra::Point3::from_slice(eye)); self }
  /// Specify the target position as a slice `[x, y, z]`
  pub fn target(mut self, target: &[f32]) -> CameraBuilder { self.target = Some(nalgebra::Point3::from_slice(target)); self }
  /// Specify the up vector as a slice `[x, y, z]`
  pub fn up(mut self, up: &[f32])         -> CameraBuilder { self.up = Some(nalgebra::Vector3::from_row_slice(up)); self }

  /// Create a `Camera` object from the builder parameters. Returns an error if parameters have not been specified.
  pub fn into(self) -> Result<Camera, JsError> {
  /*
    let view: nalgebra::Matrix4<f32> = nalgebra::Isometry3::<f32>::look_at_rh(
      &self.eye.ok_or("Eye not specified")?,
      &self.target.ok_or("Target not specified")?,
      &self.up.ok_or("Up not specified")?,
    ).to_homogeneous();
    */
    Ok(Camera::new(
      self.width.ok_or("Width not specified")?,
      self.height.ok_or("Height not specified")?,
      self.fov.ok_or("fov not specified")?,
      self.eye.ok_or("eye not specified")?,
      self.target.ok_or("target not specified")?,
      self.up.ok_or("up not specified")?,
      // distance: (self.target.ok_or("Target not specified")? - self.eye.ok_or("Eye not specified")?).norm(),
      )?
    )
  }
}


