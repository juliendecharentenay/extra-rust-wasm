
#[cfg(feature = "wasm")]
/// Type employed for error returned within functions that may be exposed to JavaScript using `wasm-bindgen`
pub type JsError = wasm_bindgen::JsValue;

#[cfg(not(feature = "wasm"))]
/// Type employed for error returned within functions that may be exposed to JavaScript using `wasm-bindgen`
pub type JsError = Box<dyn std::error::Error>;

#[derive(Debug)]
/// Error that encapsulate a dynamic error to faciliate the exposition of error to JavaScript
pub struct Error {
  e: Box<dyn std::error::Error>,
}

impl From<std::num::TryFromIntError> for Error {
  fn from(e: std::num::TryFromIntError) -> Error { e.to_string().into() }
}

impl From<derive_builder::UninitializedFieldError> for Error {
  fn from(e: derive_builder::UninitializedFieldError) -> Error { e.to_string().into() }
}

impl From<Box<dyn std::error::Error>> for Error {
  fn from(e: Box<dyn std::error::Error>) -> Error { Error { e } }
}

impl From<&str> for Error {
  fn from(e: &str) -> Error { 
    let e: Box<dyn std::error::Error> = e.into();
    Error { e }
  }
}

impl From<String> for Error {
  fn from(e: String) -> Error { e.as_str().into() }
}

#[cfg(feature = "wasm")]
impl From<Error> for wasm_bindgen::JsValue {
  fn from(error: Error) -> wasm_bindgen::JsValue {
    wasm_bindgen::JsValue::from_str(format!("{}", error.e).as_str())
  }
}


impl From<Error> for Box<dyn std::error::Error> {
  fn from(error: Error) -> Box<dyn std::error::Error> {
    error.e
  }
}

