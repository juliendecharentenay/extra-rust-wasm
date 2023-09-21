use super::*;

mod rendererbuilder; pub use rendererbuilder::RendererBuilder;
mod programlines;
mod programtriangleswithnormals;

pub enum Info<'a> {
  Lines(&'a Vec<(nalgebra::Point3<f32>, nalgebra::Point3<f32>)>),
  TrianglesWithNormals {
    vertices: &'a Vec<(nalgebra::Point3<f32>, nalgebra::Point3<f32>, nalgebra::Point3<f32>)>,
    normals:  &'a Vec<(nalgebra::Vector3<f32>, nalgebra::Vector3<f32>, nalgebra::Vector3<f32>)>
  },
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
/// Struct to managing rendering. Hold consistent copy of `Camera`, WebGL programs etc and provides interface
/// for rendering lines and triangles
pub struct Renderer {
  camera: Camera,
  program_lines: std::rc::Rc<std::cell::RefCell<Option<web_sys::WebGlProgram>>>,
  program_triangles_with_normals: std::rc::Rc<std::cell::RefCell<Option<web_sys::WebGlProgram>>>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl Renderer {
  /// Update `Camera` component in place and returns the `Renderer`
  pub fn with_camera(mut self, camera: Camera) -> Self { self.camera = camera; self }
}

impl Renderer {
  /// Create a new `Renderer` object. It is private to only allow use of builder pattern
  fn new(camera: Camera,
    program_lines: std::rc::Rc<std::cell::RefCell<Option<web_sys::WebGlProgram>>>,
    program_triangles_with_normals: std::rc::Rc<std::cell::RefCell<Option<web_sys::WebGlProgram>>>,
    ) -> Renderer { 
    Renderer { camera, program_lines, program_triangles_with_normals, } 
  }

  /// Draw information provided
  pub fn draw(&self, 
    context: &web_sys::WebGl2RenderingContext,
    info: Info
    ) -> Result<(), Error> {
    match info {
      Info::Lines(vertices) => {
        if self.program_lines.borrow().is_none() {
          *self.program_lines.borrow_mut() = Some(programlines::ProgramLines::webgl_program(context)?);
        }
        let program = self.program_lines.borrow();
        let program = program.as_ref().ok_or("Unable to retrieve program to draw lines...")?;
        context.use_program(Some(program));

        // Bind vertices
        let positions = vertices.iter()
        .fold(Vec::new(), |mut r, (s, e)| {r.push(s); r.push(e); r});
        let positions = Self::point3_to_vecf32(&positions);
        Self::bind(context, program, "vPosition", &positions)?;

        // Bind camera
        let u_matrix = context.get_uniform_location(program, "uMatrix");
        context.uniform_matrix4fv_with_f32_array(u_matrix.as_ref(), false, self.camera.as_matrix()?.as_slice());

        let n: i32 = (2*vertices.len()).try_into()?;
        context.draw_arrays(web_sys::WebGl2RenderingContext::LINES, 0, n);
      },

      Info::TrianglesWithNormals { vertices, normals } => {
        if self.program_triangles_with_normals.borrow().is_none() {
          *self.program_triangles_with_normals.borrow_mut() = Some(programtriangleswithnormals::ProgramTrianglesWithNormals::webgl_program(context)?);
        }
        let program = self.program_triangles_with_normals.borrow();
        let program = program.as_ref().ok_or("Unable to retrieve program to draw lines...")?;
        context.use_program(Some(program));

        // Bind vertices
        let positions = vertices.iter()
        .fold(Vec::new(), |mut r, (p1, p2, p3)| {r.push(p1); r.push(p2); r.push(p3); r});
        let positions = Self::point3_to_vecf32(&positions);
        Self::bind(context, program, "a_position", &positions)?;

        // Bind normals
        let normals = normals.iter()
        .fold(Vec::new(), |mut r, (n1, n2, n3)| {r.push(n1); r.push(n2); r.push(n3); r});
        let normals = Self::vector3_to_vecf32(&normals);
        Self::bind(context, program, "a_normal", &normals)?;
        
        // Set color
        let u_color = context.get_uniform_location(program, "u_color");
        context.uniform4f(u_color.as_ref(), 0.3, 0.3, 0.3, 1.0);

        // Set camera
        let u_worldview_projection = context.get_uniform_location(program, "uWorldviewProjection");
        context.uniform_matrix4fv_with_f32_array(u_worldview_projection.as_ref(), false, self.camera.as_matrix()?.as_slice());
        let u_worldview = context.get_uniform_location(program, "uWorldviewInverseTranspose");
        context.uniform_matrix4fv_with_f32_array(u_worldview.as_ref(), false, self.camera.as_transpose_inverse_view_matrix()?.as_slice());

        // Set lighting
        let u_reverse_light_direction = context.get_uniform_location(program, "u_reverseLightDirection");
        context.uniform3f(u_reverse_light_direction.as_ref(), 1.0, 1.0, 1.0);

        let n: i32 = (3*vertices.len()).try_into()?;
        context.draw_arrays(web_sys::WebGl2RenderingContext::TRIANGLES, 0, n);
      },
    };

    Ok(())
  }

  /// Bind the values of an array to the context using the key provided.
  /// Assumes that the program has already been set.
  /// Note: makes a copy of the array in place of the unsafe view
  /// into the array...
  fn bind(
    context: &web_sys::WebGl2RenderingContext,
    program: &web_sys::WebGlProgram,
    key: &str,
    array: &Vec<f32>
  ) -> Result<(), Error> {
    let buffer = context.create_buffer().ok_or("Unable to create buffer")?;
    context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    /*
    unsafe {
      let view = js_sys::Float32Array::view(array.as_slice());
      context.buffer_data_with_array_buffer_view(
        web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
        &view,
        web_sys::WebGl2RenderingContext::STATIC_DRAW);
    }
    */
    let view = js_sys::Float32Array::new_with_length(array.len().try_into()?);
    view.copy_from(array.as_slice());
    context.buffer_data_with_array_buffer_view(
        web_sys::WebGl2RenderingContext::ARRAY_BUFFER,
        &view,
        web_sys::WebGl2RenderingContext::STATIC_DRAW);

    let position = context.get_attrib_location(program, key);
    context.vertex_attrib_pointer_with_i32(position.try_into()?, 3, web_sys::WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(position as u32);
    context.bind_buffer(web_sys::WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    Ok(())
  }

  // Convert an array of `nalgebra::Point3<f32>` into a `Vec<f32>`
  fn point3_to_vecf32(array: &Vec<&nalgebra::Point3<f32>>) -> Vec<f32> {
    array.iter()
    .map(|p| vec!(p.x, p.y, p.z))
    .fold(Vec::new(), |mut r, mut a| {r.append(&mut a); r})
  }

  // Convert an array of `nalgebra::Vector3<f32>` into a `Vec<f32>`
  fn vector3_to_vecf32(array: &Vec<&nalgebra::Vector3<f32>>) -> Vec<f32> {
    array.iter()
    .map(|p| vec!(p.x, p.y, p.z))
    .fold(Vec::new(), |mut r, mut a| {r.append(&mut a); r})
  }
}

