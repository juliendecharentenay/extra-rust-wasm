use super::*;

pub mod hexahedronbuilder;

/// UID initialisation function
fn nano_id() -> String { nanoid::nanoid!(6) }

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Hexahedron {
  #[serde(default = "nano_id")]
  uid:   String,
  start: nalgebra::Point3<f32>,
  end:   nalgebra::Point3<f32>,
  transform: Vec<transform::Transform>,
}

impl Transformable for Hexahedron {
  fn with_transform(mut self, transform: transform::Transform) -> Self {
    self.transform.push(transform); self
  }

  fn transform_iter(&self) -> std::slice::Iter<'_, transform::Transform> {
    self.transform.iter()
  }
}
  

/*
impl Hexahedron {
  fn transform(mut self, transform: transform::Transform) -> Self {
    self.transform.push(transform); self
  }
}
*/

impl Hexahedron {
  pub const TYPE_NAME: &str = "Hexahedron";
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl Hexahedron {
  fn new(
    start: nalgebra::Point3<f32>,
    end: nalgebra::Point3<f32>) -> Result<Hexahedron, Error> {
    Ok( Hexahedron { uid: nanoid::nanoid!(6),
          start, end, 
          transform: Vec::new(),
          } )
  }

  /// Retrieve the object id
  /// Exposed to JavaScript
  pub fn uuid(&self) -> Result<String, JsError> {
    Ok(self.uid.clone())
  }

  /// Retrieve type name
  /// Exposed to JavaScript
  pub fn type_name(&self) -> Result<String, JsError> {
    Ok(Self::TYPE_NAME.to_string())
  }

  /// Apply a translation. Exposed to JavaScript
  pub fn with_translate(self, translate: transform::translation::Translation) -> Self {
    self.with_transform(translate.into())
  }

  /// Apply a translation. Exposed to JavaScript
  pub fn translate(self, translate: transform::translation::Translation) -> Self {
    self.with_transform(translate.into())
  }

  /// Draw the hex on the context
  /// Exposed to JavaScript
  pub fn draw(&self, context: &web_sys::WebGl2RenderingContext, renderer: &renderer::Renderer) -> Result<(), JsError> {
    Drawable::draw(self, context, renderer)
  }
  pub fn clone(&self) -> Self {
    Clone::clone(self)
  }
}

impl Identifiable for Hexahedron {
  /// Retrieve the object uuid
  fn uuid(&self) -> Result<String, Error> {
    Ok(self.uid.clone())
  }
}

impl Drawable for Hexahedron {
  /// Draw the hex on the context
  fn draw<T>(&self, context: &web_sys::WebGl2RenderingContext, renderer: &T) -> Result<(), JsError> 
  where T: renderer::RendererTrait {
    let (vertices, normals) = self.vertices()?;
    let (vertices, normals): (Vec<(_, _, _)>, Vec<(_, _, _)>) = std::iter::zip(vertices.into_iter(), normals.into_iter())
      .map(|((p1, p2, p3), n)| 
        Ok(
          ((self.transform_point(&p1)?, self.transform_point(&p2)?, self.transform_point(&p3)?),
           (self.transform_vector(&p1, &n)?, self.transform_vector(&p2, &n)?, self.transform_vector(&p3, &n)?))
        )
      )
      .collect::<Result<Vec<((nalgebra::Point3<f32>, nalgebra::Point3<f32>, nalgebra::Point3<f32>), 
                             (nalgebra::Vector3<f32>, nalgebra::Vector3<f32>, nalgebra::Vector3<f32>))>, Error>>()?
      .into_iter()
      .unzip();
    let info = renderer::Info::TrianglesWithNormals {
      uid: &self.uid,
      vertices: &vertices,
      normals: &normals,
    };
    Ok( renderer.draw(context, info)? )
  }
}

#[cfg(feature = "wasm")]
impl Hexahedron {
  fn vertices(&self) -> Result<(Vec<(nalgebra::Point3<f32>, nalgebra::Point3<f32>, nalgebra::Point3<f32>)>, Vec<nalgebra::Vector3<f32>>), Error> {
    use nalgebra::Vector3;

    let delta = self.end - self.start;
    let delta_x = delta.dot(&Vector3::x())*Vector3::x();
    let delta_y = delta.dot(&Vector3::y())*Vector3::y();
    let delta_z = delta.dot(&Vector3::z())*Vector3::z();

    let mut v = Vec::new(); let mut n = Vec::new();
    // Triangle 1
    v.push((self.start, self.start + delta_y, self.start + delta_x)); 
    n.push(-1.0f32*Vector3::z());
    // Triangle 2
    v.push((self.start + delta_x, self.start + delta_y, self.start + delta_x + delta_y));
    n.push(-1.0f32*Vector3::z());

    // Triangle 3
    v.push((self.start, self.start + delta_x, self.start + delta_z));
    n.push(-1.0f32*Vector3::y());
    // Triangle 4
    v.push((self.start + delta_z, self.start + delta_x, self.start + delta_x + delta_z));
    n.push(-1.0f32*Vector3::y());

    // Triangle 5
    v.push((self.start, self.start + delta_z, self.start + delta_y));
    n.push(-1.0f32*Vector3::x());
    // Triangle 6
    v.push((self.start + delta_y, self.start + delta_z, self.start + delta_y + delta_z));
    n.push(-1.0f32*Vector3::x());

    // Triangle 7
    v.push(( self.start + delta_z + delta_x, self.start + delta_z + delta_y, self.start + delta_z, )); 
    n.push(1.0f32*Vector3::z());
    // Triangle 8
    v.push(( self.start + delta_z + delta_x + delta_y, self.start + delta_z + delta_y, self.start + delta_z + delta_x, ));
    n.push(1.0f32*Vector3::z());

    // Triangle 9
    v.push(( self.start + delta_y + delta_z, self.start + delta_y + delta_x, self.start + delta_y, ));
    n.push(1.0f32*Vector3::y());
    // Triangle 10
    v.push(( self.start + delta_y + delta_x + delta_z, self.start + delta_y + delta_x, self.start + delta_y + delta_z, ));
    n.push(1.0f32*Vector3::y());

    // Triangle 11
    v.push((self.start + delta_x, self.start + delta_x + delta_y, self.start + delta_x + delta_z));
    n.push(1.0f32*Vector3::x());
    // Triangle 12
    v.push((self.start + delta_x + delta_z, self.start + delta_x + delta_y, self.start + delta_x + delta_y + delta_z));
    n.push(1.0f32*Vector3::x());

    Ok((v, n))
  }
}

