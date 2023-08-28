use super::*;

#[cfg(feature = "wasm")]
#[derive(Default)]
pub struct WebGlProgramBuilder<'a> {
  context: Option<&'a web_sys::WebGl2RenderingContext>,
  vertex_shader_source: Option<&'a str>,
  fragment_shader_source: Option<&'a str>,
}

#[cfg(feature = "wasm")]
impl<'a> WebGlProgramBuilder<'a> {
  /// Create a new empty `ProgramBuilder`
  pub fn new() -> WebGlProgramBuilder<'a> {
    WebGlProgramBuilder {
      context: None,
      vertex_shader_source: None,
      fragment_shader_source: None,
    }
  }

  /// Specify the WebGL context
  pub fn context(mut self, context: &'a web_sys::WebGl2RenderingContext) -> WebGlProgramBuilder<'a> { self.context = Some(context); self }

  /// Specify the vertex shader source
  pub fn vertex_shader_source(mut self, vertex_shader_source: &'a str) -> WebGlProgramBuilder<'a> { self.vertex_shader_source = Some(vertex_shader_source); self }

  /// Specify the fragment shader source
  pub fn fragment_shader_source(mut self, fragment_shader_source: &'a str) -> WebGlProgramBuilder<'a> { self.fragment_shader_source = Some(fragment_shader_source); self }

  /// Build the program
  pub fn build(self) -> Result<web_sys::WebGlProgram, Error> {
    let context = self.context.ok_or("WebGl rendering context not specified")?;
    let vertex_shader = self.compile_shader(context, web_sys::WebGl2RenderingContext::VERTEX_SHADER,
                              self.vertex_shader_source.ok_or("Vertex shader source not specified")?)?;

    let fragment_shader = self.compile_shader(context, web_sys::WebGl2RenderingContext::FRAGMENT_SHADER,
                              self.fragment_shader_source.ok_or("Fragment shader source not specified")?)?;

    let program = context.create_program().ok_or("Unable to create program")?;
    context.attach_shader(&program, &vertex_shader);
    context.attach_shader(&program, &fragment_shader);
    context.link_program(&program);
    if context.get_program_parameter(&program, web_sys::WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
      Ok(program)
    } else {
      Err(context.get_program_info_log(&program).unwrap_or(String::from("Error in linking program")).into())
    }
  }
}

#[cfg(feature = "wasm")]
impl<'a> WebGlProgramBuilder<'a> {
  fn compile_shader(&self,
                    context: &web_sys::WebGl2RenderingContext,
                    shader_type: u32,
                    source: &str,
                    ) -> Result<web_sys::WebGlShader, Error> {
    let shader = context.create_shader(shader_type).ok_or("Unable to create shader object")?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);
    if context
        .get_shader_parameter(&shader, web_sys::WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) {
        Ok(shader)
    } else {
        Err(context.get_shader_info_log(&shader).unwrap_or(String::from("Unknown error")).into())
    }
  }
}

