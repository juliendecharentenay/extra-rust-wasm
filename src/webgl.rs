use super::*;

mod webglprogrambuilder; pub use webglprogrambuilder::WebGlProgramBuilder;

pub mod renderer; use renderer::Renderer;
pub mod camera; pub use camera::Camera;
pub mod grid; pub use grid::Grid;
pub mod hexahedron; pub use hexahedron::Hexahedron;
mod transform;
mod traits; pub use traits::{Drawable};

/// List of drawable elements - ie elements that implements the drawable trait
#[derive(serde::Serialize, serde::Deserialize)]
#[enum_dispatch::enum_dispatch(Drawable)]
pub enum DrawableElement {
  Grid(grid::Grid),
  Hexahedron(hexahedron::Hexahedron),
}

#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "export function try_as_grid(v) { return v; }")]
extern "C" {
  #[wasm_bindgen::prelude::wasm_bindgen(catch)]
  fn try_as_grid(v: wasm_bindgen::JsValue) -> Result<grid::Grid, wasm_bindgen::JsValue>;
}

#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "export function try_as_hexahedron(v) { return v; }")]
extern "C" {
  #[wasm_bindgen::prelude::wasm_bindgen(catch)]
  fn try_as_hexahedron(v: wasm_bindgen::JsValue) -> Result<hexahedron::Hexahedron, wasm_bindgen::JsValue>;
}

#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "export function get_type_name(v) { return v.type_name(); }")]
extern "C" {
  #[wasm_bindgen::prelude::wasm_bindgen(catch)]
  fn get_type_name(v: &wasm_bindgen::JsValue) -> Result<String, wasm_bindgen::JsValue>;
}

#[derive(thiserror::Error, Debug)]
pub enum WebglError {
  #[error("Item `{0}` is not a supported drawable element.")]
  DrawableUnsupported(String),
  #[error("An error occured when converting to drawable element.")]
  DrawableError,
}

impl std::convert::TryFrom<wasm_bindgen::JsValue> for DrawableElement {
  type Error = WebglError;
  fn try_from(v: wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
    let name = get_type_name(&v).map_err(|e| { web_sys::console::error_1(&e); WebglError::DrawableError })?;
    match name.as_str() {
      grid::Grid::TYPE_NAME => try_as_grid(v).map(|r| r.into()).map_err(|e| { web_sys::console::error_1(&e); WebglError::DrawableError }),
      hexahedron::Hexahedron::TYPE_NAME => try_as_hexahedron(v).map(|r| r.into()).map_err(|e| { web_sys::console::error_1(&e); WebglError::DrawableError }),
      _ => Err(WebglError::DrawableUnsupported(name)),
    }
  }
}

