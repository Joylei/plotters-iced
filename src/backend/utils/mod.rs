// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

pub(crate) mod converter;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod path;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod shape;
