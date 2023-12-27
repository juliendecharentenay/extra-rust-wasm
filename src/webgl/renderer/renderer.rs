use super::*;

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
  pub fn new(camera: Camera,
    program_lines: std::rc::Rc<std::cell::RefCell<Option<web_sys::WebGlProgram>>>,
    program_triangles_with_normals: std::rc::Rc<std::cell::RefCell<Option<web_sys::WebGlProgram>>>,
    ) -> Renderer { 
    Renderer { camera, program_lines, program_triangles_with_normals, } 
  }
}

impl RendererTrait for Renderer {
  /// Initialise render
  fn init(&self, context: &web_sys::WebGl2RenderingContext) -> Result<(), Error> {
    context.viewport(0, 0, self.camera.get_width() as i32, self.camera.get_height() as i32);
    context.clear_color(6.0/255.0, 78.0/255.0, 59.0/255.0, 1.0);
    context.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT | web_sys::WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    context.enable(web_sys::WebGl2RenderingContext::CULL_FACE);
    context.enable(web_sys::WebGl2RenderingContext::DEPTH_TEST);
    Ok(())
  }

  /// Draw information provided
  fn draw(&self, 
    context: &web_sys::WebGl2RenderingContext,
    info: Info
    ) -> Result<(), Error> {
    match info {
      Info::Lines { uid: _, vertices } => {
        if self.program_lines.borrow().is_none() {
          *self.program_lines.borrow_mut() = Some(programlines::ProgramLines::webgl_program(context)?);
        }
        let program = self.program_lines.borrow();
        let program = program.as_ref().ok_or("Unable to retrieve program to draw lines...")?;
        context.use_program(Some(program));

        // Bind vertices
        let positions = vertices.iter()
        .fold(Vec::new(), |mut r, (s, e)| {r.push(s); r.push(e); r});
        let positions = utils::point3_to_vecf32(&positions);
        utils::bind(context, program, "vPosition", &positions)?;

        // Bind camera
        let u_matrix = context.get_uniform_location(program, "uMatrix");
        context.uniform_matrix4fv_with_f32_array(u_matrix.as_ref(), false, self.camera.as_matrix()?.as_slice());

        let n: i32 = (2*vertices.len()).try_into()?;
        context.draw_arrays(web_sys::WebGl2RenderingContext::LINES, 0, n);
      },

      Info::TrianglesWithNormals { uid: _, vertices, normals } => {
        if self.program_triangles_with_normals.borrow().is_none() {
          *self.program_triangles_with_normals.borrow_mut() = Some(programtriangleswithnormals::ProgramTrianglesWithNormals::webgl_program(context)?);
        }
        let program = self.program_triangles_with_normals.borrow();
        let program = program.as_ref().ok_or("Unable to retrieve program to draw lines...")?;
        context.use_program(Some(program));

        // Bind vertices
        let positions = vertices.iter()
        .fold(Vec::new(), |mut r, (p1, p2, p3)| {r.push(p1); r.push(p2); r.push(p3); r});
        let positions = utils::point3_to_vecf32(&positions);
        utils::bind(context, program, "a_position", &positions)?;

        // Bind normals
        let normals = normals.iter()
        .fold(Vec::new(), |mut r, (n1, n2, n3)| {r.push(n1); r.push(n2); r.push(n3); r});
        let normals = utils::vector3_to_vecf32(&normals);
        utils::bind(context, program, "a_normal", &normals)?;
        
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
}


