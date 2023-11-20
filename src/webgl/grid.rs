use super::*;

pub mod gridbuilder;

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Grid {
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
    Ok( Grid { normal, tangent, center, delta, n, } )
  }

  /// Retrieve type name
  pub fn type_name(&self) -> Result<String, JsError> {
    Ok(Self::TYPE_NAME.to_string())
  }

  /// Draw the grid on the context
  pub fn draw(&self, context: &web_sys::WebGl2RenderingContext, renderer: &renderer::Renderer) -> Result<(), JsError> {
    Drawable::draw(self, context, renderer)
  }
}

impl Drawable for Grid {
  /// Draw the grid on the context
  fn draw(&self, context: &web_sys::WebGl2RenderingContext, renderer: &renderer::Renderer) -> Result<(), JsError> {
    let vertices = self.vertices()?;
    let info = renderer::Info::Lines(&vertices);
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

