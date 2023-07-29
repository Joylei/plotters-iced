// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
/// Indicates that some error occurred within the Iced backend
pub enum Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl StdError for Error {}
