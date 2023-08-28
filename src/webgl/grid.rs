use super::*;

pub mod gridbuilder;

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct Grid {
  program: web_sys::WebGlProgram,
  normal:  nalgebra::Vector3<f32>,
  tangent: nalgebra::Vector3<f32>,
  center:  nalgebra::Point3<f32>,
  delta:   f32,
  n:       u32,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl Grid {
  fn new(context: &web_sys::WebGl2RenderingContext,
         normal:   nalgebra::Vector3<f32>,
         tangent:  nalgebra::Vector3<f32>,
         center:   nalgebra::Point3<f32>,
         delta:    f32,
         n:        u32) -> Result<Grid, Error>
  {
    let program = WebGlProgramBuilder::new()
      .context(context)
      .vertex_shader_source(Grid::VERTEX_SHADER_SOURCE)
      .fragment_shader_source(Grid::FRAGMENT_SHADER_SOURCE)
      .build()?;
    Ok( Grid { program, normal, tangent, center, delta, n, } )
  }

  /// Draw the grid on the context
  pub fn draw(&self, context: &web_sys::WebGl2RenderingContext, camera: Vec<f32>) -> Result<(), JsError> {
    let vertices = self.vertices()?;
    self.bind_vertex(context, "vPosition", &vertices)?;
    self.redraw(context, camera)
  }

  /// Redraw the grid on the context on a camera change (but no change of data)
  pub fn redraw(&self, context: &web_sys::WebGl2RenderingContext, camera: Vec<f32>) -> Result<(), JsError> {
    Ok(self.redraw_impl(context, camera)?)
  }

  fn redraw_impl(&self, context: &web_sys::WebGl2RenderingContext, camera: Vec<f32>) -> Result<(), Error> {
    context.use_program(Some(&self.program));
    let u_matrix = context.get_uniform_location(&self.program, "uMatrix");
    context.uniform_matrix4fv_with_f32_array(u_matrix.as_ref(), false, camera.as_slice());
    let n: i32 = (4*(self.n+1)).try_into()?;
    context.draw_arrays(web_sys::WebGl2RenderingContext::LINES, 0, n);
    Ok(())
  }
}

#[cfg(feature = "wasm")]
impl Grid {
  fn vertices(&self) -> Result<Vec<f32>, Error> {
    let n = self.normal.normalize();
    let t = self.tangent.normalize();
    let c: nalgebra::Vector3<f32> = n.cross(&t);
    let l = (self.n as f32) * self.delta;
    let vertices = (0..=self.n).map(|i| { -0.5*l + (i as f32) * self.delta })
    .fold(Vec::new(),
      |mut r, x| {
        let p = self.center - x*t - 0.5*l*c;
        r.push(p.x); r.push(p.y); r.push(p.z);
        let p = self.center - x*t + 0.5*l*c;
        r.push(p.x); r.push(p.y); r.push(p.z);
        let p = self.center - x*c - 0.5*l*t;
        r.push(p.x); r.push(p.y); r.push(p.z);
        let p = self.center - x*c + 0.5*l*t;
        r.push(p.x); r.push(p.y); r.push(p.z);
        r
      });
    Ok(vertices)
  }

  fn bind_vertex(&self,
          context: &web_sys::WebGl2RenderingContext,
          key: &str,
          array: &Vec<f32>) -> Result<(), Error>
  {
    let buffer = context.create_buffer().ok_or("Unable to create buffer")?;
    context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    unsafe {
      let view = js_sys::Float32Array::view(array.as_slice());
      context.buffer_data_with_array_buffer_view(
        web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
        &view,
        web_sys::WebGl2RenderingContext::STATIC_DRAW);
    }
    let va = context.create_vertex_array().ok_or("Unable to create vertex array")?;
    context.bind_vertex_array(Some(&va));

    context.vertex_attrib_pointer_with_i32(0, 3, web_sys::WebGl2RenderingContext::FLOAT, false, 0, 0);
    let position = context.get_attrib_location(&self.program, key);
    context.enable_vertex_attrib_array(position as u32);
    
    context.bind_vertex_array(Some(&va));
    Ok(())
  }
}

#[cfg(feature = "wasm")]
impl Grid {
  const VERTEX_SHADER_SOURCE: &str = r#"
     attribute vec4 vPosition;
     uniform mat4 uMatrix;
     void main()
     {
        gl_Position = uMatrix*vPosition;
     }
    "#;

  const FRAGMENT_SHADER_SOURCE: &str = r#"
    precision mediump float;
     void main()
     {
       gl_FragColor = vec4(0.9, 0.9, 0.9, 1);
     }
    "#;
}


