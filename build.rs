// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if !target.starts_with("wasm32") {
        build_poly2tri();
    }
}

fn build_poly2tri() {
    cc::Build::new()
        .cpp(true)
        .include("vendor/poly2tri")
        .file("src/native/backend/triangulate/binding.cpp")
        .compile("libpoly2tri.a");
}
