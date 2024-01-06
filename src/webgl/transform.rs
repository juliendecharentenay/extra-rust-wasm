use super::*;

pub mod identity;
pub mod translation;
pub mod scaling;
pub mod rotation;
// mod transformation;

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Transform {
  Identity(identity::Identity),
  Translate(translation::Translation),
  Scale(scaling::Scaling),
  Rotate(rotation::Rotation),
  // Transform(transformation::Transformation),
}

impl From<identity::Identity> for Transform {
  fn from(v: identity::Identity) -> Transform { Transform::Identity(v) }
}

impl From<translation::Translation> for Transform {
  fn from(v: translation::Translation) -> Transform { Transform::Translate(v) }
}

impl From<scaling::Scaling> for Transform {
  fn from(v: scaling::Scaling) -> Transform { Transform::Scale(v) }
}

impl From<rotation::Rotation> for Transform {
  fn from(v: rotation::Rotation) -> Transform { Transform::Rotate(v) }
}

impl Transform {
/*
  pub fn new_identity() -> Transform { Transform::Identity(identity::Identity::default()) }

  pub fn new_translation(x: f32, y: f32, z: f32) -> Transform { 
    Transform::Translate(translation::Translation::new(x, y, z))
  }
  */

  pub fn transform_point(&self, p: &nalgebra::Point3<f32>) -> Result<nalgebra::Point3<f32>, Error> {
    Ok(self.matrix4_at(p)?.transform_point(p))
  }

  pub fn transform_vector(&self, p: &nalgebra::Point3<f32>, v: &nalgebra::Vector3<f32>) -> Result<nalgebra::Vector3<f32>, Error> {
    Ok(self.matrix4_at(p)?.transform_vector(v))
  }

  fn matrix4_at(&self, _p: &nalgebra::Point3<f32>) -> Result<nalgebra::Matrix4<f32>, Error> {
    match self {
      Transform::Identity(op)  => Ok(op.matrix4()),
      Transform::Translate(op) => Ok(op.matrix4()),
      Transform::Scale(op)     => Ok(op.matrix4()),
      Transform::Rotate(op)    => Ok(op.matrix4()),
    }
  }
}

