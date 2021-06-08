// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

fn main() {
    build_poly2tri();
}

fn build_poly2tri() {
    cc::Build::new()
        .cpp(true)
        .include("vendor/poly2tri")
        .file("src/triangulate/binding.cpp")
        .compile("libpoly2tri.a");
}
