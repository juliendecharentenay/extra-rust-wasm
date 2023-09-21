use super::*;

pub mod hexahedronbuilder;

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct Hexahedron {
  start: nalgebra::Point3<f32>,
  end:   nalgebra::Point3<f32>,
  transform: Vec<transform::Transform>,
}

impl Hexahedron {
  fn transform(mut self, transform: transform::Transform) -> Self {
    self.transform.push(transform); self
  }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl Hexahedron {
  fn new(
    start: nalgebra::Point3<f32>,
    end: nalgebra::Point3<f32>) -> Result<Hexahedron, Error> {
    Ok( Hexahedron { start, end, 
          transform: Vec::new(),
          } )
  }

  pub fn translate(self, translate: transform::translation::Translation) -> Self {
    self.transform(translate.into())
  }

  fn transform_point(&self, p: &nalgebra::Point3<f32>) -> Result<nalgebra::Point3<f32>, Error> {
    self.transform.iter().fold(Ok(p.clone()), |p, t| p.and_then(|p| t.transform_point(&p)))
  }

  fn transform_vector(&self, p: &nalgebra::Point3<f32>, v: &nalgebra::Vector3<f32>) -> Result<nalgebra::Vector3<f32>, Error> {
    Ok(self.transform.iter().fold(Ok((p.clone(), v.clone())), 
      |r: Result<(_, _), Error>, t| r.and_then(|(p, v)| Ok((t.transform_point(&p)?, t.transform_vector(&p, &v)?)))
      )?.1)
  }

  /// Draw the hex on the context
  pub fn draw(&self, context: &web_sys::WebGl2RenderingContext, renderer: &renderer::Renderer) -> Result<(), JsError> {
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
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(format!("{:?}", vertices).as_str()));
      /*
    let normals = normals.into_iter()
      .map(|n| {(n, n, n)})
      .collect::<Vec<(nalgebra::Vector3<f32>, nalgebra::Vector3<f32>, nalgebra::Vector3<f32>)>>();
      */
    let info = renderer::Info::TrianglesWithNormals {
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

