// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

#[cfg(target_arch = "wasm32")]
use js_sys::JSON;
use std::error::Error as StdError;
use std::fmt;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[derive(Debug)]
/// Indicates that some error occurred within the Iced backend
pub enum Error {
    /// web backend error
    #[cfg(target_arch = "wasm32")]
    Web(String),
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl StdError for Error {}

#[cfg(target_arch = "wasm32")]
impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Self::Web(
            JSON::stringify(&e)
                .map(|s| s.into())
                .unwrap_or_else(|_| "Unknown".to_string()),
        )
    }
}
