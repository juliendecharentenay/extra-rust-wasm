use super::*;

pub mod gridbuilder;

/// UID initialisation function
fn nano_id() -> String { nanoid::nanoid!(6) }

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Grid {
  #[serde(default = "nano_id")]
  uid:     String,
  normal:  nalgebra::Vector3<f32>,
  tangent: nalgebra::Vector3<f32>,
  center:  nalgebra::Point3<f32>,
  delta:   f32,
  n:       u32,
}

impl Grid {
  pub const TYPE_NAME: &str = "Grid";
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl Grid {
  fn new(normal:   nalgebra::Vector3<f32>,
         tangent:  nalgebra::Vector3<f32>,
         center:   nalgebra::Point3<f32>,
         delta:    f32,
         n:        u32) -> Result<Grid, Error>
  {
    Ok( Grid { uid: nanoid::nanoid!(6), normal, tangent, center, delta, n, } )
  }

  /// Retrieve the object id
  /// Exposed to JavaScript
  pub fn uuid(&self) -> Result<String, JsError> {
    Ok(self.uid.clone())
  }

  /// Retrieve type name.
  /// Exposed to JavaScript
  pub fn type_name(&self) -> Result<String, JsError> {
    Ok(Self::TYPE_NAME.to_string())
  }

  /// Draw the grid on the context
  /// Exposed to JavaScript
  pub fn draw(&self, context: &web_sys::WebGl2RenderingContext, renderer: &renderer::Renderer) -> Result<(), JsError> {
    Drawable::draw(self, context, renderer)
  }
}

impl Drawable for Grid {
  /// Draw the grid on the context
  fn draw<T>(&self, context: &web_sys::WebGl2RenderingContext, renderer: &T) -> Result<(), JsError> 
  where T: renderer::RendererTrait {
    let vertices = self.vertices()?;
    let info = renderer::Info::Lines {
      uid: &self.uid,
      vertices: &vertices 
    };
    Ok( renderer.draw(context, info)? )
  }
}

#[cfg(feature = "wasm")]
impl Grid {
  fn vertices(&self) -> Result<Vec<(nalgebra::Point3<f32>, nalgebra::Point3<f32>)>, Error> {
    let n = self.normal.normalize();
    let t = self.tangent.normalize();
    let c: nalgebra::Vector3<f32> = n.cross(&t);
    let l = (self.n as f32) * self.delta;
    let vertices = (0..=self.n).map(|i| { -0.5*l + (i as f32) * self.delta })
    .fold(Vec::new(),
      |mut r, x| {
        let s = self.center - x*t - 0.5*l*c;
        let e = self.center - x*t + 0.5*l*c;
        r.push((s,e));
        let s = self.center - x*c - 0.5*l*t;
        let e = self.center - x*c + 0.5*l*t;
        r.push((s,e));
        r
      });
    Ok(vertices)
  }
}

