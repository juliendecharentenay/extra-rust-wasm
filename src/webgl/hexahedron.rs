use super::*;

pub mod hexahedronbuilder;

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct Hexahedron {
  program: std::rc::Rc<std::cell::RefCell<Option<web_sys::WebGlProgram>>>,
  start: nalgebra::Point3<f32>,
  end:   nalgebra::Point3<f32>,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl Hexahedron {
  fn new_without_context(start: nalgebra::Point3<f32>,
    end: nalgebra::Point3<f32>) -> Result<Hexahedron, Error> {
    Ok( Hexahedron { program: std::rc::Rc::new(std::cell::RefCell::new(None)),
                     start, end } )
  }

  /// Retrieve the program (compile if required)
  fn program_rc(&self, context: &web_sys::WebGl2RenderingContext) -> Result<std::rc::Rc<std::cell::RefCell<Option<web_sys::WebGlProgram>>>, Error> {
    if self.program.borrow().is_none() {
      *self.program.borrow_mut() = Some(WebGlProgramBuilder::new()
        .context(context)
        .vertex_shader_source(Hexahedron::VERTEX_SHADER_SOURCE)
        .fragment_shader_source(Hexahedron::FRAGMENT_SHADER_SOURCE)
        .build()?);
    }
    Ok(std::rc::Rc::clone(&self.program))
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
    let program_rc     = self.program_rc(context)?;
    let program_borrow = program_rc.borrow();
    let program        = program_borrow.as_ref().ok_or("Program not initialized")?;
    context.use_program(Some(program));
    let u_matrix = context.get_uniform_location(program, "uMatrix");
    context.uniform_matrix4fv_with_f32_array(u_matrix.as_ref(), false, camera.as_slice());
    let n: i32 = 3*2*6; // .try_into()?;
    context.draw_arrays(web_sys::WebGl2RenderingContext::TRIANGLES, 0, n);
    Ok(())
  }
}

#[cfg(feature = "wasm")]
impl Hexahedron {
  fn vertices(&self) -> Result<Vec<f32>, Error> {
    use nalgebra::Vector3;

    let delta = self.end - self.start;
    let delta_x = delta.dot(&Vector3::x())*Vector3::x();
    let delta_y = delta.dot(&Vector3::y())*Vector3::y();
    let delta_z = delta.dot(&Vector3::z())*Vector3::z();

    let mut v = Vec::new();
    // Triangle 1
    v.append(&mut Self::vector3_to_slice(&self.start)); 
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_x)));
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_y)));
    // Triangle 2
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_y))); 
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_x)));
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_x + delta_y)));

    // Triangle 3
    v.append(&mut Self::vector3_to_slice(&self.start)); 
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_x)));
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_z)));
    // Triangle 4
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_z))); 
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_x)));
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_x + delta_z)));

    // Triangle 5
    v.append(&mut Self::vector3_to_slice(&self.start)); 
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_y)));
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_z)));
    // Triangle 6
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_z))); 
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_y)));
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_y + delta_z)));

    // Triangle 7
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_z))); 
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_z + delta_x)));
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_z + delta_y)));
    // Triangle 8
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_z + delta_y))); 
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_z + delta_x)));
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_z + delta_x + delta_y)));

    // Triangle 9
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_y))); 
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_y + delta_x)));
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_y + delta_z)));
    // Triangle 10
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_y + delta_z))); 
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_y + delta_x)));
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_y + delta_x + delta_z)));

    // Triangle 11
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_x))); 
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_x + delta_y)));
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_x + delta_z)));
    // Triangle 12
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_x + delta_z))); 
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_x + delta_y)));
    v.append(&mut Self::vector3_to_slice(&(self.start + delta_x + delta_y + delta_z)));

    Ok(v)
  }

  fn vector3_to_slice(v: &nalgebra::Point3<f32>) -> Vec<f32> {
    vec!(v.x, v.y, v.z)
  }

  fn bind_vertex(&self,
          context: &web_sys::WebGl2RenderingContext,
          key: &str,
          array: &Vec<f32>) -> Result<(), Error>
  {
    let program_rc     = self.program_rc(context)?;
    let program_borrow = program_rc.borrow();
    let program        = program_borrow.as_ref().ok_or("Program not initialized")?;
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
    let position = context.get_attrib_location(program, key);
    context.enable_vertex_attrib_array(position as u32);

    context.bind_vertex_array(Some(&va));
    Ok(())
  }
}

#[cfg(feature = "wasm")]
impl Hexahedron {
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
       gl_FragColor = vec4(0.6, 0.6, 0.6, 1);
     }
    "#;
}

