use super::*;

pub mod builder;

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
/// Struct for managing object picking
pub struct Picker {
  camera: Camera,
  program: std::rc::Rc<std::cell::RefCell<Option<web_sys::WebGlProgram>>>,
  objects: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
  pick_position: Option<(i32, i32)>,
  pick_result: std::rc::Rc<std::cell::RefCell<Option<String>>>,
}

impl Picker {
  const N_RGBA_VALUES: usize = 255;
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl Picker {
  /// Update `Camera` component in place and return updated object
  pub fn with_camera(mut self, camera: Camera) -> Self { self.camera = camera; self }

  /// Set position to be tested for picking
  pub fn with_pick_position(mut self, client_x: i32, client_y: i32) -> Self { self.pick_position = Some((client_x, client_y)); self }

  /// Retrieve result
  pub fn result(&self) -> Option<String> { self.pick_result.borrow().clone() }
}

impl Picker {
  pub fn from_camera(camera: Camera) -> Picker {
    Picker { 
      camera, 
      program: std::rc::Rc::new(std::cell::RefCell::new(None)),
      objects: std::rc::Rc::new(std::cell::RefCell::new(Vec::new())),
      pick_position: None,
      pick_result: std::rc::Rc::new(std::cell::RefCell::new(None)),
    }
  }
}

impl RendererTrait for Picker {
  /// Initialise render
  fn init(&self,
    context: &web_sys::WebGl2RenderingContext,
    ) -> Result<(), Error> {

    let texture = context.create_texture().ok_or("Unable to create texture")?;
    context.bind_texture(web_sys::WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
    context.tex_parameteri(web_sys::WebGl2RenderingContext::TEXTURE_2D, web_sys::WebGl2RenderingContext::TEXTURE_MIN_FILTER, web_sys::WebGl2RenderingContext::LINEAR.try_into()?);
    context.tex_parameteri(web_sys::WebGl2RenderingContext::TEXTURE_2D, web_sys::WebGl2RenderingContext::TEXTURE_WRAP_S, web_sys::WebGl2RenderingContext::CLAMP_TO_EDGE.try_into()?);
    context.tex_parameteri(web_sys::WebGl2RenderingContext::TEXTURE_2D, web_sys::WebGl2RenderingContext::TEXTURE_WRAP_T, web_sys::WebGl2RenderingContext::CLAMP_TO_EDGE.try_into()?);

    let depth_buffer = context.create_renderbuffer().ok_or("Unable to create render buffer")?;
    context.bind_renderbuffer(web_sys::WebGl2RenderingContext::RENDERBUFFER, Some(&depth_buffer));

    context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
      web_sys::WebGl2RenderingContext::TEXTURE_2D,       // target
      0,                                                 // Level
      web_sys::WebGl2RenderingContext::RGBA.try_into()?, // internal format
      self.camera.get_width() as i32,                    // width
      self.camera.get_height() as i32,                   // height
      0,                                                 // border
      web_sys::WebGl2RenderingContext::RGBA,             // format
      web_sys::WebGl2RenderingContext::UNSIGNED_BYTE,    // Type
      None,                                              // data
    )?;
    context.bind_renderbuffer(web_sys::WebGl2RenderingContext::RENDERBUFFER, Some(&depth_buffer));
    context.renderbuffer_storage(web_sys::WebGl2RenderingContext::RENDERBUFFER, web_sys::WebGl2RenderingContext::DEPTH_COMPONENT16,
      self.camera.get_width() as i32, self.camera.get_height() as i32);


    let frame_buffer = context.create_framebuffer().ok_or("Unable to create frame buffer")?;
    context.bind_framebuffer(web_sys::WebGl2RenderingContext::FRAMEBUFFER, Some(&frame_buffer));

    context.framebuffer_texture_2d(web_sys::WebGl2RenderingContext::FRAMEBUFFER,
      web_sys::WebGl2RenderingContext::COLOR_ATTACHMENT0,
      web_sys::WebGl2RenderingContext::TEXTURE_2D,
      Some(&texture),
      0,       // level
    );
    context.framebuffer_renderbuffer(web_sys::WebGl2RenderingContext::FRAMEBUFFER,
      web_sys::WebGl2RenderingContext::DEPTH_ATTACHMENT,
      web_sys::WebGl2RenderingContext::RENDERBUFFER,
      Some(&depth_buffer)
    );

    context.viewport(0, 0, self.camera.get_width() as i32, self.camera.get_height() as i32);
    context.enable(web_sys::WebGl2RenderingContext::CULL_FACE);
    context.enable(web_sys::WebGl2RenderingContext::DEPTH_TEST);
    context.clear_color(0.0, 0.0, 0.0, 0.0);
    context.clear(web_sys::WebGl2RenderingContext::COLOR_BUFFER_BIT | web_sys::WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    *self.objects.borrow_mut() = Vec::new();
    Ok(())
  }

  /// Draw information provided
  fn draw(&self, 
    context: &web_sys::WebGl2RenderingContext,
    info: Info
    ) -> Result<(), Error> {
    // Retrieve program (initialise if required)
    if self.program.borrow().is_none() {
      *self.program.borrow_mut() = Some(ProgramPicker::webgl_program(context)?);
    }
    let program = self.program.borrow();
    let program = program.as_ref().ok_or("Unable to retrieve picker program...")?;
    context.use_program(Some(program));

    // Bind camera
    /*
    let u_world = context.get_uniform_location(program, "u_world");
    context.uniform_matrix4fv_with_f32_array(u_world.as_ref(), false, self.camera.as_world_matrix()?.as_slice());
    let u_view_projection = context.get_uniform_location(program, "u_viewProjection");
    context.uniform_matrix4fv_with_f32_array(u_view_projection.as_ref(), false, self.camera.as_world_matrix()?.as_slice());
    */
    let u_matrix = context.get_uniform_location(program, "uMatrix");
    context.uniform_matrix4fv_with_f32_array(u_matrix.as_ref(), false, self.camera.as_matrix()?.as_slice());
    
    // Set color based on id counter
    let count = self.objects.borrow().len() + 1;
    let n = Picker::N_RGBA_VALUES;
    let r = count.rem_euclid(n); let count = count.div_euclid(n);
    let g = count.rem_euclid(n); let count = count.div_euclid(n);
    let b = count.rem_euclid(n); let count = count.div_euclid(n);
    let a = count.rem_euclid(n);
    let u_id = context.get_uniform_location(program, "u_id");
    context.uniform4f(u_id.as_ref(), r as f32/n as f32, g as f32/n as f32, b as f32/n as f32, a as f32/n as f32);

    match info {
      Info::Lines { uid, vertices } => {
        self.objects.borrow_mut().push(uid.clone());

        // Bind vertices
        let positions = vertices.iter()
        .fold(Vec::new(), |mut r, (s, e)| {r.push(s); r.push(e); r});
        let positions = utils::point3_to_vecf32(&positions);
        utils::bind(context, program, "a_position", &positions)?;

        // Draw
        let n: i32 = (2*vertices.len()).try_into()?;
        context.draw_arrays(web_sys::WebGl2RenderingContext::LINES, 0, n);

      },
      Info::TrianglesWithNormals { uid, vertices, normals: _ } => {
        self.objects.borrow_mut().push(uid.clone());

        // Bind vertices
        let positions = vertices.iter()
        .fold(Vec::new(), |mut r, (p1, p2, p3)| {r.push(p1); r.push(p2); r.push(p3); r});
        let positions = utils::point3_to_vecf32(&positions);
        utils::bind(context, program, "a_position", &positions)?;

        // Draw
        let n: i32 = (3*vertices.len()).try_into()?;
        context.draw_arrays(web_sys::WebGl2RenderingContext::TRIANGLES, 0, n);
      },
    };
    Ok(())
  }

  /// Post render
  fn end(&self, context: &web_sys::WebGl2RenderingContext) -> Result<(), Error> {
    if self.pick_position.is_some() {
      let p = self.pick_position.as_ref().unwrap();
      let mut data: [u8; 4] = [0; 4];
      let r = context.read_pixels_with_opt_u8_array(
        p.0, // x
        p.1, // y
        1,   // width
        1,   // height
        web_sys::WebGl2RenderingContext::RGBA, // format
        web_sys::WebGl2RenderingContext::UNSIGNED_BYTE, // Type
        Some(&mut data), // dst_data
      )?;
      let n = Picker::N_RGBA_VALUES as u8;
      let count = data[0] + n*(data[1] + n*(data[2] + n*data[3]));
      let count = count as usize;
      if count > self.objects.borrow().len() { return Err(format!("Found object {count} - which exceeds object length {}", self.objects.borrow().len()).into()); }
      *self.pick_result.borrow_mut() = if count == 0 { 
        None 
      } else {
        Some(self.objects.borrow()[count-1].clone())
      };
    }
    context.bind_framebuffer(web_sys::WebGl2RenderingContext::FRAMEBUFFER, None);
    Ok(())
  }
}

pub struct ProgramPicker {}

impl ProgramPicker {
  pub fn webgl_program(
    context: &web_sys::WebGl2RenderingContext,
  ) -> Result<web_sys::WebGlProgram, Error> {
    Ok(WebGlProgramBuilder::new()
      .context(context)
      .vertex_shader_source(Self::VERTEX_SHADER_SOURCE)
      .fragment_shader_source(Self::FRAGMENT_SHADER_SOURCE)
      .build()?)
  }

  const VERTEX_SHADER_SOURCE: &str = r#"
  attribute vec4 a_position;
  
  // uniform mat4 u_viewProjection;
  // uniform mat4 u_world;
  uniform mat4 uMatrix;
  
  void main() {
    // Multiply the position by the matrix.
    // gl_Position = u_viewProjection * u_world * a_position;
    gl_Position = uMatrix * a_position;
  }
  "#;

  const FRAGMENT_SHADER_SOURCE: &str = r#"
  precision mediump float;
  
  uniform vec4 u_id;

  void main() {
     gl_FragColor = u_id;
  }
  "#;
}
