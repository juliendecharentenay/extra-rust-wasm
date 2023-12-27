use super::*;

/// Trait for object that can be drawn
#[enum_dispatch::enum_dispatch]
pub trait Drawable {
  fn draw<T>(&self, context: &web_sys::WebGl2RenderingContext, renderer: &T) -> Result<(), JsError> where T: renderer::RendererTrait ;
}
