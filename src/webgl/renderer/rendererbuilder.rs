use super::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
#[derive(Default)]
/// Builder pattern for `Renderer` struct
pub struct RendererBuilder {
  camera: Option<Camera>,
  program_lines: Option<web_sys::WebGlProgram>,
  program_triangles_with_normals: Option<web_sys::WebGlProgram>,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl RendererBuilder {
  /// Create an empty `RendererBuilder`
  pub fn new() -> RendererBuilder { RendererBuilder::default() }

/*
  /// Create a `RendererBuilder` from a `Renderer` object to allow a new `Renderer` object to be created
  pub fn from_renderer(renderer: Renderer) -> RendererBuilder {
    RendererBuilder {
      camera: Some(renderer.camera),
      program_lines: renderer.program_lines.clone().into_inner(),
      program_triangles_with_normals: renderer.program_triangles_with_normals.clone().into_inner(),
    }
  }
  */
   
  /// Specify a `Camera` object
  pub fn camera(mut self, camera: Camera) -> RendererBuilder { self.camera = Some(camera); self }
  
  /// Build a `Renderer` object
  pub fn build(self) -> Result<Renderer, JsError> {
    Ok(
      Renderer::new(
        self.camera.ok_or("Camera is not specified")?,
        std::rc::Rc::new(std::cell::RefCell::new(self.program_lines)),
        std::rc::Rc::new(std::cell::RefCell::new(self.program_triangles_with_normals)),
      )
    )
  }
}

