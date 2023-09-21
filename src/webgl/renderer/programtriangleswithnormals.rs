use super::*;

pub struct ProgramTrianglesWithNormals { }

impl ProgramTrianglesWithNormals {
  pub fn webgl_program(
    context: &web_sys::WebGl2RenderingContext
    ) -> Result<web_sys::WebGlProgram, Error> {
    Ok(WebGlProgramBuilder::new()
      .context(context)
      .vertex_shader_source(Self::VERTEX_SHADER_SOURCE)
      .fragment_shader_source(Self::FRAGMENT_SHADER_SOURCE)
      .build()?)
  }

  const VERTEX_SHADER_SOURCE: &str = r#"
     attribute vec4 a_position;
     attribute vec3 a_normal;
     uniform mat4 uWorldviewInverseTranspose;

     uniform mat4 uWorldviewProjection;

     varying vec3 vNormal;

     void main()
     {
        gl_Position = uWorldviewProjection*a_position;
        vNormal = mat3(uWorldviewInverseTranspose)*a_normal;
     }
    "#;


  const FRAGMENT_SHADER_SOURCE: &str = r#"
     precision mediump float;

     varying vec3 vNormal;

     uniform vec3 u_reverseLightDirection;

     uniform vec4 u_color;

     void main()
     {
       gl_FragColor = u_color;
       vec3 normal = normalize(vNormal);
       float light = (dot(normal, u_reverseLightDirection)+2.0)/3.0;
       gl_FragColor.rgb *= light;
     }
    "#;


}
