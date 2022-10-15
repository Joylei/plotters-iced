use std::ops::Index;

/// https://github.com/jeromefroe/lttb-rs

pub trait DataPoint {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

impl<D: DataPoint> DataPoint for &D {
    fn x(&self) -> f64 {
        DataPoint::x(*self)
    }

    fn y(&self) -> f64 {
        DataPoint::y(*self)
    }
}

pub trait LttbSource {
    type Item;

    fn len(&self) -> usize;

    fn item_at(&self, i: usize) -> Self::Item;

    fn cast<T, F>(self, f: F) -> Cast<Self, T, F>
    where
        Self: Sized,
        T: DataPoint,
        F: Fn(Self::Item) -> T,
    {
        Cast { s: self, f }
    }

    /// lttb sample
    fn lttb(self, threshold: usize) -> LttbIterator<Self>
    where
        Self: Sized,
        Self::Item: DataPoint,
    {
        let is_sample = !(threshold >= self.len() || threshold == 0);
        let len = self.len();
        LttbIterator {
            source: self,
            is_sample,
            idx: 0,
            a: 0,
            threshold,
            every: if is_sample {
                ((len - 2) as f64) / ((threshold - 2) as f64)
            } else {
                0_f64
            },
        }
    }
}

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

    fn len(&self) -> usize {
        self.s.len()
    }

    fn item_at(&self, i: usize) -> Self::Item {
        let item = self.s.item_at(i);
        (&self.f)(item)
    }
}

impl<'a, S: LttbSource> LttbSource for &'a S {
    type Item = S::Item;

    fn len(&self) -> usize {
        S::len(&self)
    }

    fn item_at(&self, i: usize) -> Self::Item {
        S::item_at(&self, i)
    }
}

pub struct LttbIterator<S: LttbSource> {
    source: S,
    is_sample: bool,
    idx: usize,
    a: usize,
    threshold: usize,
    every: f64,
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
                    let idx = (avg_range_start + i) as usize;
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
                    let idx = (range_offs + i) as usize;

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
}

impl<'a, T> LttbSource for &'a [T] {
    type Item = &'a T;
    #[inline]
    fn len(&self) -> usize {
        #[inline]
        fn slice_len<T>(a: &[T]) -> usize {
            a.len()
        }
        slice_len(self)
    }
    #[inline]
    fn item_at(&self, i: usize) -> Self::Item {
        &self[i]
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
