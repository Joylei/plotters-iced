// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

pub(crate) type PathScalar = i32;

type PathSimplifierPointInner = [PathScalar; 2];
type PathSimplifierPointOuter = [f64; 2];

enum PathSimplifierGroup {
    None,
    X(PathScalar),
    Y(PathScalar),
}

pub(crate) struct PathSimplifier<I: Iterator<Item = PathSimplifierPointInner>> {
    source_points: I,
    current_group: PathSimplifierGroup,
    last_point: Option<PathSimplifierPointInner>,
}

impl<I: Iterator<Item = PathSimplifierPointInner>> PathSimplifier<I> {
    pub(crate) fn from(source_points: I) -> Self {
        Self {
            source_points,
            current_group: PathSimplifierGroup::None,
            last_point: None,
        }
    }
}

impl<I: Iterator<Item = PathSimplifierPointInner>> Iterator for PathSimplifier<I> {
    type Item = PathSimplifierPointOuter;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Branch to source points iterator (exhaust next group)
        for point in &mut self.source_points {
            // Backtrack in points
            // Retain current point as 'last point'
            if let Some(point_before) = self.last_point.replace(point) {
                // De-duplicate points
                if point_before != point {
                    let mut do_yield = false;

                    match self.current_group {
                        PathSimplifierGroup::None => {
                            // Start a new group from last closed group? (backtrack on X or Y)
                            if point_before[0] == point[0] {
                                self.current_group = PathSimplifierGroup::X(point_before[0]);
                            } else if point_before[1] == point[1] {
                                self.current_group = PathSimplifierGroup::Y(point_before[1]);
                            }

                            // Yield start-of-group or isolated point
                            do_yield = true;
                        }
                        PathSimplifierGroup::X(opener_x) => {
                            // Close current X group? (using 'before' point)
                            if point[0] != opener_x {
                                // Start a new Y group immediately? (immediate backtrack on Y)
                                // Notice: this is an edge case which prevents the next start of \
                                //   group to be skipped in cases where the last group intersects \
                                //   with the next group, on a point on the segment (ie. not on \
                                //   its edges).
                                if point_before[1] == point[1] {
                                    self.current_group = PathSimplifierGroup::Y(point_before[1]);
                                } else {
                                    self.current_group = PathSimplifierGroup::None;
                                }

                                // Yield end-of-group point
                                do_yield = true;
                            }
                        }
                        PathSimplifierGroup::Y(opener_y) => {
                            // Close current Y group? (using 'before' point)
                            if point[1] != opener_y {
                                // Start a new X group immediately? (immediate backtrack on X)
                                // Notice: this is an edge case which prevents the next start of \
                                //   group to be skipped in cases where the last group intersects \
                                //   with the next group, on a point on the segment (ie. not on \
                                //   its edges).
                                if point_before[0] == point[0] {
                                    self.current_group = PathSimplifierGroup::X(point_before[0]);
                                } else {
                                    self.current_group = PathSimplifierGroup::None;
                                }

                                // Yield end-of-group point
                                do_yield = true;
                            }
                        }
                    }
                    if do_yield {
                        return Some([point_before[0] as f64, point_before[1] as f64]);
                    }
                }
            }
        }

        // End of the source points iterator, close path? (this yields)
        if let Some(last_point) = self.last_point {
            self.last_point = None;

            return Some([last_point[0] as f64, last_point[1] as f64]);
        }

        // Done painting all path points
        None
    }
}
