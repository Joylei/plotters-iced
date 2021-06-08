// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

use num_traits::identities::{One, Zero};

type ShapeSplitterValue = f64;
type ShapeSplitterPoint = [ShapeSplitterValue; 2];

pub(crate) struct ShapeSplitter {
    last_point: ShapeSplitterPoint,
    path_segments: Vec<[ShapeSplitterPoint; 2]>,
}

impl ShapeSplitter {
    pub(crate) fn try_from(path: &[ShapeSplitterPoint]) -> Result<Self, ()> {
        // Only proceed if we have enough points to form at least a triangle
        if path.len() >= 3 {
            // Map all unique segments for the simplified path
            let mut path_segments = Vec::new();

            for index in 0..(path.len() - 1) {
                let (current_point, next_point) = (path[index], path[index + 1]);

                path_segments.push([current_point, next_point]);
            }

            Ok(Self {
                last_point: path[path.len() - 1],
                path_segments,
            })
        } else {
            Err(())
        }
    }

    #[allow(clippy::float_cmp)]
    pub(crate) fn collect(&mut self) -> Vec<Vec<ShapeSplitterPoint>> {
        let (mut closed_shapes, mut current_shape_index) = (vec![Vec::new()], 0);

        // Intersect each segment with all of its following segments, iteratively, and \
        //   create paths for closed shapes.
        for index in 0..self.path_segments.len() {
            let path_segment = &self.path_segments[index];

            // Push opening (or current) point?
            // Notice: check that previously pushed point does not match opening point, as in some \
            //   cases the last pushed intersection point would be equal to the opening point to \
            //   push there.
            Self::append_point(&mut closed_shapes[current_shape_index], path_segment[0]);

            for sibling_index in (index + 1)..self.path_segments.len() {
                let sibling_path_segment = &self.path_segments[sibling_index];

                // The lines are not directly connected? Proceed with intersection check.
                if path_segment[1] != sibling_path_segment[0]
                    && path_segment[0] != sibling_path_segment[1]
                {
                    let intersection = Self::intersects(path_segment, sibling_path_segment);

                    // An intersection has been found, the current shape can be closed and yielded
                    if let Some(point_intersect) = intersection {
                        // Close current closed shape at this point (ensure we are not pushing an \
                        //   intersection equal to the starting point right after it)
                        Self::append_point(
                            &mut closed_shapes[current_shape_index],
                            point_intersect,
                        );

                        // Start a new shape at this point (will be closed upon a future iteration)
                        closed_shapes.push(vec![point_intersect]);

                        current_shape_index += 1;
                    }
                }
            }
        }

        // Close the first shape with the last point from the original shape?
        // Notice: points shall not be repeated, hence why there is a check that we are not \
        //   closing the shape with either its starting point, or already-there ending point.
        if !closed_shapes.is_empty() {
            Self::append_point(&mut closed_shapes[0], self.last_point);
        }

        closed_shapes
    }

    #[allow(clippy::many_single_char_names, clippy::float_cmp)]
    fn intersects(
        line: &[ShapeSplitterPoint; 2],
        other: &[ShapeSplitterPoint; 2],
    ) -> Option<ShapeSplitterPoint> {
        // Adapted from: https://github.com/ucarion/line_intersection/blob/master/src/lib.rs#L108

        let p = line[0];
        let q = other[0];
        let r = [line[1][0] - line[0][0], line[1][1] - line[0][1]];
        let s = [other[1][0] - other[0][0], other[1][1] - other[0][1]];

        let r_cross_s = Self::point_cross(&r, &s);
        let q_minus_p = [q[0] - p[0], q[1] - p[1]];

        // Lines parallel? Ignore.
        if r_cross_s == ShapeSplitterValue::zero() {
            None
        } else {
            // Lines are not parallel, continue.
            let t = Self::point_cross(&q_minus_p, &Self::point_divide(&s, r_cross_s));
            let u = Self::point_cross(&q_minus_p, &Self::point_divide(&r, r_cross_s));

            // Do the lines intersect in one point?
            let t_in_range = ShapeSplitterValue::zero() <= t && t <= ShapeSplitterValue::one();
            let u_in_range = ShapeSplitterValue::zero() <= u && u <= ShapeSplitterValue::one();

            if t_in_range && u_in_range {
                // Return intersection point coordinates (rounded as to avoid floating point errors)
                Some([(p[0] + t * r[0]).round(), (p[1] + t * r[1]).round()])
            } else {
                None
            }
        }
    }

    #[inline(always)]
    fn point_cross(point: &ShapeSplitterPoint, other: &ShapeSplitterPoint) -> ShapeSplitterValue {
        point[0] * other[1] - point[1] * other[0]
    }

    #[inline(always)]
    fn point_divide(point: &ShapeSplitterPoint, other: ShapeSplitterValue) -> ShapeSplitterPoint {
        [point[0] / other, point[1] / other]
    }

    #[inline(always)]
    #[allow(clippy::float_cmp)]
    fn append_point(container: &mut Vec<ShapeSplitterPoint>, point: ShapeSplitterPoint) {
        let container_size = container.len();

        if container_size == 0 || (container[container_size - 1] != point && container[0] != point)
        {
            container.push(point);
        }
    }
}
