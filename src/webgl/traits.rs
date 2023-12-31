use super::*;

/// Trait for object that can be identified by a uuid
#[enum_dispatch::enum_dispatch]
pub trait Identifiable {
  fn uuid(&self) -> Result<String, Error>;
}

/// Trait for object that can be drawn
#[enum_dispatch::enum_dispatch]
pub trait Drawable {
  fn draw<T>(&self, context: &web_sys::WebGl2RenderingContext, renderer: &T) -> Result<(), JsError> where T: renderer::RendererTrait ;
}

/// Trait for transforming objects
#[enum_dispatch::enum_dispatch]
pub trait Transformable {
  /// Append a transform to the list of transformations
  fn with_transform(self, transform: transform::Transform) -> Self;

  /// Return an iterator on the list of transformations
  fn transform_iter(&self) -> std::slice::Iter<'_, transform::Transform>;

  /// Apply the transformations to a 3D point coordinate
  fn transform_point(&self, p: &nalgebra::Point3<f32>) -> Result<nalgebra::Point3<f32>, Error> {
    self.transform_iter().fold(Ok(p.clone()), |p, t| p.and_then(|p| t.transform_point(&p)))
  }

  /// Apply the transformations to a 3D vector at a given point
  fn transform_vector(&self, p: &nalgebra::Point3<f32>, v: &nalgebra::Vector3<f32>) -> Result<nalgebra::Vector3<f32>, Error> {
    Ok(self.transform_iter().fold(Ok((p.clone(), v.clone())), 
      |r: Result<(_, _), Error>, t| r.and_then(|(p, v)| Ok((t.transform_point(&p)?, t.transform_vector(&p, &v)?)))
      )?.1)
  }
}
