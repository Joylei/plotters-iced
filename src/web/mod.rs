// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

mod svg;
mod text;
mod widget;

use dodrio::bumpalo;
use plotters_backend::BackendColor;
use wasm_bindgen::prelude::*;
pub use widget::ChartWidget;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// helper
trait AsBumpStr {
    fn as_bump_str<'b>(&self, bump: &'b bumpalo::Bump) -> &'b str;
}

macro_rules! impl_as_bump_str {
    ($t:ident) => {
        impl AsBumpStr for $t {
            #[inline(always)]
            fn as_bump_str<'b>(&self, bump: &'b bumpalo::Bump) -> &'b str {
                bumpalo::format!(
                    in bump, "{}", self)
                .into_bump_str()
            }
        }
    };
}

impl_as_bump_str!(u16);
impl_as_bump_str!(i32);
impl_as_bump_str!(u32);
impl_as_bump_str!(f64);
impl AsBumpStr for &str {
    #[inline(always)]
    fn as_bump_str<'b>(&self, bump: &'b bumpalo::Bump) -> &'b str {
        bumpalo::format!(
            in bump, "{}", self)
        .into_bump_str()
    }
}

impl AsBumpStr for BackendColor {
    #[inline(always)]
    fn as_bump_str<'b>(&self, bump: &'b bumpalo::Bump) -> &'b str {
        let (r, g, b) = self.rgb;
        bumpalo::format!(
            in bump, "#{:02X}{:02X}{:02X}", r,g,b)
        .into_bump_str()
    }
}
