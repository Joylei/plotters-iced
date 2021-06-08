// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

// This module was imported from rust-poly2tri (https://github.com/mitsuhiko/rust-poly2tri), and \
//   improved so that memory leaks were all fixed (especially regarding vector contents \
//   de-allocation, which was not done properly).

extern crate libc;

use libc::{c_void, size_t};
use std::mem;

type Scalar = f64;

extern "C" {
    fn p2t_polyline_new() -> *mut c_void;
    fn p2t_polyline_free(polygon: *mut c_void);
    fn p2t_polyline_add_point(polygon: *mut c_void, x: Scalar, y: Scalar);

    fn p2t_cdt_new(polygon: *mut c_void) -> *mut c_void;
    fn p2t_cdt_free(cdt: *mut c_void);
    fn p2t_cdt_triangulate(cdt: *mut c_void);
    fn p2t_cdt_get_triangles(cdt: *mut c_void) -> *mut c_void;

    fn p2t_triangles_free(triangles: *mut c_void);
    fn p2t_triangles_count(triangles: *mut c_void) -> size_t;
    fn p2t_triangles_get_triangle(triangles: *const c_void, idx: size_t) -> *const c_void;

    fn p2t_triangle_get_point(
        triangle: *const c_void,
        idx: size_t,
        x_out: *mut Scalar,
        y_out: *mut Scalar,
    );
}

pub(crate) struct Polygon {
    ll: *mut c_void,
}

pub(crate) struct CDT {
    ll: *mut c_void,
}

pub(crate) struct TriangleVec {
    ll: *mut c_void,

    #[allow(dead_code)]
    cdt: CDT,
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) struct Triangle {
    pub(crate) points: [[Scalar; 2]; 3],
}

impl Polygon {
    pub(crate) fn new() -> Polygon {
        unsafe {
            Polygon {
                ll: p2t_polyline_new(),
            }
        }
    }

    pub(crate) fn from_iterator<'a, I>(points: I) -> Polygon
    where
        I: Iterator<Item = &'a [Scalar; 2]>,
    {
        let mut rv = Polygon::new();

        for point in points {
            rv.add_point(point[0], point[1]);
        }

        rv
    }

    pub(crate) fn add_point(&mut self, x: Scalar, y: Scalar) {
        unsafe {
            p2t_polyline_add_point(self.ll, x, y);
        }
    }
}

impl Drop for Polygon {
    fn drop(&mut self) {
        unsafe {
            p2t_polyline_free(self.ll);
        }
    }
}

impl CDT {
    pub(crate) fn new(polygon: Polygon) -> CDT {
        unsafe {
            let rv = CDT {
                ll: p2t_cdt_new(polygon.ll),
            };

            mem::forget(polygon);

            rv
        }
    }

    pub(crate) fn triangulate(self) -> TriangleVec {
        unsafe {
            p2t_cdt_triangulate(self.ll);

            let ll = p2t_cdt_get_triangles(self.ll);

            TriangleVec { cdt: self, ll }
        }
    }
}

impl Drop for CDT {
    fn drop(&mut self) {
        unsafe {
            p2t_cdt_free(self.ll);
        }
    }
}

impl TriangleVec {
    pub(crate) fn size(&self) -> usize {
        unsafe { p2t_triangles_count(self.ll) as usize }
    }

    pub(crate) fn get_triangle(&self, idx: usize) -> Triangle {
        assert!(idx < self.size(), "Out of range");

        let mut p0 = [0.0; 2];
        let mut p1 = [0.0; 2];
        let mut p2 = [0.0; 2];

        unsafe {
            let tri = p2t_triangles_get_triangle(self.ll, idx as size_t);

            p2t_triangle_get_point(tri, 0, &mut p0[0], &mut p0[1]);
            p2t_triangle_get_point(tri, 1, &mut p1[0], &mut p1[1]);
            p2t_triangle_get_point(tri, 2, &mut p2[0], &mut p2[1]);
        }

        Triangle {
            points: [p0, p1, p2],
        }
    }
}

impl Drop for TriangleVec {
    fn drop(&mut self) {
        unsafe {
            p2t_triangles_free(self.ll);
        }
    }
}

pub(crate) fn triangulate_points<'a, I>(points: I) -> TriangleVec
where
    I: Iterator<Item = &'a [Scalar; 2]>,
{
    CDT::new(Polygon::from_iterator(points)).triangulate()
}
