// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT
//

//! Largest-Triangle-Three-Buckets algorithm (LTTB)
//!
//! ## Known limitations
//! - X-values must be in a strictly increasing order

// original version: https://github.com/sveinn-steinarsson/flot-downsample
// modified based on https://github.com/jeromefroe/lttb-rs

/// data point for [`LttbSource`]
pub trait DataPoint {
    /// x value for sampling, must be in a strictly increasing order
    fn x(&self) -> f64;
    /// y value for sampling
    fn y(&self) -> f64;
}

impl<D: DataPoint> DataPoint for &D {
    #[inline]
    fn x(&self) -> f64 {
        (*self).x()
    }
    #[inline]
    fn y(&self) -> f64 {
        (*self).y()
    }
}

/// data source for lttb sampling
///
/// ## Known limitations
/// - X-values must be in a strictly increasing order
pub trait LttbSource {
    /// data item of [`LttbSource`]
    type Item;

    /// length of [`LttbSource`]
    fn len(&self) -> usize;

    /// is [`LttbSource`] empty
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// data item at  index `i`
    fn item_at(&self, i: usize) -> Self::Item;

    /// map data item to another type.
    /// - if the data item type of [`LttbSource`] is not [`DataPoint`], lttb sampling can be used after casting
    fn cast<T, F>(self, f: F) -> Cast<Self, T, F>
    where
        Self: Sized,
        T: DataPoint,
        F: Fn(Self::Item) -> T,
    {
        Cast { s: self, f }
    }

    /// lttb sampling
    fn lttb(self, threshold: usize) -> LttbIterator<Self>
    where
        Self: Sized,
        Self::Item: DataPoint,
    {
        let is_sample = !(threshold >= self.len() || threshold < 3);
        let every = if is_sample {
            ((self.len() - 2) as f64) / ((threshold - 2) as f64)
        } else {
            0_f64
        };
        LttbIterator {
            source: self,
            is_sample,
            idx: 0,
            a: 0,
            threshold,
            every,
        }
    }
}

/// map data item to another type
pub struct Cast<S, T, F>
where
    S: LttbSource,
    T: DataPoint,
    F: Fn(S::Item) -> T,
{
    s: S,
    f: F,
}

impl<S, T, F> LttbSource for Cast<S, T, F>
where
    S: LttbSource,
    T: DataPoint,
    F: Fn(S::Item) -> T,
{
    type Item = T;
    #[inline]
    fn len(&self) -> usize {
        self.s.len()
    }

    #[inline]
    fn item_at(&self, i: usize) -> Self::Item {
        let item = self.s.item_at(i);
        (self.f)(item)
    }
}

impl<'a, S: LttbSource> LttbSource for &'a S {
    type Item = S::Item;
    #[inline]
    fn len(&self) -> usize {
        (*self).len()
    }

    #[inline]
    fn item_at(&self, i: usize) -> Self::Item {
        (*self).item_at(i)
    }
}

/// iterator for [`LttbSource`]
pub struct LttbIterator<S: LttbSource> {
    source: S,
    is_sample: bool,
    idx: usize,
    threshold: usize,
    every: f64,
    a: usize,
}

impl<S: LttbSource> LttbIterator<S>
where
    S::Item: DataPoint,
{
    fn next_no_sample(&mut self) -> Option<S::Item> {
        if self.idx < self.source.len() {
            let item = self.source.item_at(self.idx);
            self.idx += 1;
            Some(item)
        } else {
            None
        }
    }

    fn next_sample(&mut self) -> Option<S::Item> {
        if self.idx < self.threshold {
            if self.idx == 0 {
                self.idx += 1;
                Some(self.source.item_at(0))
            } else if self.idx + 1 == self.threshold {
                self.idx += 1;
                Some(self.source.item_at(self.source.len() - 1))
            } else {
                let every = self.every;
                let i = self.idx - 1;
                // Calculate point average for next bucket (containing c).
                let mut avg_x = 0f64;
                let mut avg_y = 0f64;

                let avg_range_start = (((i + 1) as f64) * every) as usize + 1;

                let mut end = (((i + 2) as f64) * every) as usize + 1;
                if end >= self.source.len() {
                    end = self.source.len();
                }
                let avg_range_end = end;

                let avg_range_length = (avg_range_end - avg_range_start) as f64;

                for i in 0..(avg_range_end - avg_range_start) {
                    let idx = avg_range_start + i;
                    let item = self.source.item_at(idx);
                    avg_x += item.x();
                    avg_y += item.y();
                }
                avg_x /= avg_range_length;
                avg_y /= avg_range_length;

                // Get the range for this bucket.
                let range_offs = ((i as f64) * every) as usize + 1;
                let range_to = (((i + 1) as f64) * every) as usize + 1;

                // Point a.
                let item = self.source.item_at(self.a);
                let point_a_x = item.x();
                let point_a_y = item.y();

                let mut max_area = -1f64;
                let mut next_a = range_offs;
                for i in 0..(range_to - range_offs) {
                    let idx = range_offs + i;

                    // Calculate triangle area over three buckets.
                    let item = self.source.item_at(idx);
                    let area = ((point_a_x - avg_x) * (item.y() - point_a_y)
                        - (point_a_x - item.x()) * (avg_y - point_a_y))
                        .abs()
                        * 0.5;
                    if area > max_area {
                        max_area = area;
                        next_a = idx; // Next a is this b.
                    }
                }

                let item = self.source.item_at(next_a); // Pick this point from the bucket.
                self.a = next_a; // This a is the next a (chosen b).
                self.idx += 1;
                Some(item)
            }
        } else {
            None
        }
    }

    #[inline]
    fn remaining(&self) -> usize {
        if self.is_sample {
            self.threshold - self.idx
        } else {
            self.source.len() - self.idx
        }
    }
}

impl<S: LttbSource> Iterator for LttbIterator<S>
where
    S::Item: DataPoint,
{
    type Item = S::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_sample {
            self.next_sample()
        } else {
            self.next_no_sample()
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.remaining();
        (size, Some(size))
    }
}

impl<S: LttbSource> ExactSizeIterator for LttbIterator<S>
where
    S::Item: DataPoint,
{
    #[inline]
    fn len(&self) -> usize {
        self.remaining()
    }
}

impl<'a, T> LttbSource for &'a [T] {
    type Item = &'a T;
    #[inline]
    fn len(&self) -> usize {
        (*self).len()
    }

    #[inline]
    fn item_at(&self, i: usize) -> Self::Item {
        &self[i]
    }
}

impl<T: Clone> LttbSource for [T] {
    type Item = T;
    #[inline]
    fn len(&self) -> usize {
        (*self).len()
    }

    #[inline]
    fn item_at(&self, i: usize) -> Self::Item {
        self[i].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub struct DataPoint {
        pub x: f64,
        pub y: f64,
    }

    impl DataPoint {
        pub fn new(x: f64, y: f64) -> Self {
            DataPoint { x, y }
        }
    }

    impl super::DataPoint for DataPoint {
        fn x(&self) -> f64 {
            self.x
        }

        fn y(&self) -> f64 {
            self.y
        }
    }

    #[test]
    fn lttb_test() {
        let mut dps = vec![];
        dps.push(DataPoint::new(0.0, 10.0));
        dps.push(DataPoint::new(1.0, 12.0));
        dps.push(DataPoint::new(2.0, 8.0));
        dps.push(DataPoint::new(3.0, 10.0));
        dps.push(DataPoint::new(4.0, 12.0));

        let mut expected = vec![];
        expected.push(DataPoint::new(0.0, 10.0));
        expected.push(DataPoint::new(2.0, 8.0));
        expected.push(DataPoint::new(4.0, 12.0));

        let result: Vec<DataPoint> = dps.as_slice().lttb(3).cloned().collect();

        assert_eq!(expected, result);
    }
}
