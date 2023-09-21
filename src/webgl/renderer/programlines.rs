use super::*;

pub struct ProgramLines { }

impl ProgramLines {
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
