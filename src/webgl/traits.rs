use super::*;

/// Trait for object that can be drawn
#[enum_dispatch::enum_dispatch]
pub trait Drawable {
  fn draw(&self, context: &web_sys::WebGl2RenderingContext, renderer: &renderer::Renderer) -> Result<(), JsError>;
}
