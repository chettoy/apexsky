use std::{
    collections::BTreeMap,
    ops::{Bound, Range},
};

use itertools::Itertools;

#[derive(Debug, Clone, Default)]
pub struct IntervalSet<T: Ord + Copy> {
    endpoints: BTreeMap<T, bool>, // true for start, false for end
}

impl<T: Ord + Copy + std::fmt::Debug> IntervalSet<T> {
    pub fn new() -> Self {
        Self {
            endpoints: BTreeMap::new(),
        }
    }

    fn validate(&self) -> bool {
        if !self.endpoints.len().is_multiple_of(2) {
            println!("{:?}", self.endpoints);
            return false;
        }
        if self
            .endpoints
            .first_key_value()
            .is_some_and(|(_, &is_start)| !is_start)
        {
            println!("{:?}", self.endpoints);
            return false;
        }
        if self
            .endpoints
            .last_key_value()
            .is_some_and(|(_, &is_start)| is_start)
        {
            println!("{:?}", self.endpoints);
            return false;
        }
        true
    }

    pub fn clear(&mut self) {
        self.endpoints.clear();
    }

    pub fn insert(&mut self, range: Range<T>) {
        assert!(range.start <= range.end);
        if range.end == range.start {
            return;
        }

        // If it's already a subset of an interval, it doesn't need to be inserted.
        if self.get_super(range.clone()).is_some() {
            return;
        }

        let overlapping_points: Vec<(T, bool)> = self
            .endpoints
            .range((Bound::Included(range.start), Bound::Included(range.end)))
            .map(|(&point, &is_start)| (point, is_start))
            .collect();

        let begin_is_bound = if overlapping_points.first() == Some(&(range.start, true)) {
            // merge old begin bound and new begin bound
            // still the start boundary
            true
        } else if overlapping_points.first() == Some(&(range.start, false)) {
            // merge old end bound and new begin bound
            // no longer a boundary
            false
        } else {
            // If it's not in any interval it's a new boundary point
            self.get_super(range.start..range.start).is_none()
        };

        let end_is_bound = if overlapping_points.last() == Some(&(range.end, false)) {
            // merge old end bound and new end bound
            // still the end boundary
            true
        } else if overlapping_points.last() == Some(&(range.end, true)) {
            // merge new end bound and old begin bound
            // no longer a boundary
            false
        } else {
            // If it's not in any interval it's a new boundary point
            self.get_super(range.end..range.end).is_none()
        };

        for (point, _) in &overlapping_points {
            self.endpoints.remove(point);
        }

        if begin_is_bound {
            self.endpoints.insert(range.start, true);
        }
        if end_is_bound {
            self.endpoints.insert(range.end, false);
        }

        assert!(
            self.validate(),
            // "{self_clone:?}, {range:?}, {overlapping_points:?}"
        );
    }

    pub fn iter(&self) -> impl Iterator<Item = Range<T>> {
        assert!(self.validate());
        self.endpoints
            .iter()
            .tuples()
            .map(|((&p0, &p0_is_start), (&p1, &p1_is_start))| {
                // if !p0_is_start || p1_is_start {
                //     println!("{:?}", self.endpoints);
                // }
                assert!(p0_is_start);
                assert!(!p1_is_start);
                p0..p1
            })
    }

    pub fn iter_in_range(&self, range: Range<T>) -> impl Iterator<Item = Range<T>> {
        assert!(range.start <= range.end);
        assert!(self.validate());
        self.endpoints
            .range((Bound::Included(range.start), Bound::Included(range.end)))
            .skip_while(|&(_, &is_start)| !is_start)
            .tuples()
            .map(|((&p0, &p0_is_start), (&p1, &p1_is_start))| {
                assert!(p0_is_start);
                assert!(!p1_is_start);
                p0..p1
            })
    }

    pub fn get_in_range(&self, range: Range<T>) -> Vec<Range<T>> {
        self.iter_in_range(range).collect()
    }

    pub fn get_super(&self, range: Range<T>) -> Option<Range<T>> {
        assert!(range.start <= range.end);
        assert!(self.validate());
        let cursor = self.endpoints.upper_bound(Bound::Included(&range.start));

        let (&begin, &prev_is_start) = cursor.peek_prev()?;
        let (&end, &next_is_start) = cursor.peek_next()?;

        if !prev_is_start {
            return None;
        }
        assert!(!next_is_start);

        let super_range = begin..end;

        (super_range.start <= range.start && super_range.end >= range.end).then_some(super_range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_set() {
        // Create an instance of IntervalSet
        let mut set = IntervalSet::new();

        // Insert some intervals
        set.insert(1..5);
        set.insert(10..15);
        set.insert(20..25);

        // Retrieve all intervals and verify
        let intervals: Vec<_> = set.iter().collect();
        assert_eq!(intervals, vec![1..5, 10..15, 20..25]);

        assert_eq!(set.get_super(0..3), None);
        assert_eq!(set.get_super(1..1), Some(1..5));
        assert_eq!(set.get_super(5..5), None);
        assert_eq!(set.get_super(1..5), Some(1..5));
        assert_eq!(set.get_super(3..5), Some(1..5));

        assert_eq!(set.get_in_range(0..25), vec![1..5, 10..15, 20..25]);
        assert_eq!(set.get_in_range(0..3), vec![]);
        assert_eq!(set.get_in_range(1..5), vec![1..5]);
        assert_eq!(set.get_in_range(3..5), vec![]);

        // Insert an overlapping interval and merge
        set.insert(12..22);

        // Retrieve all intervals again and verify
        let intervals: Vec<_> = set.iter().collect();
        assert_eq!(intervals, vec![1..5, 10..25]);

        assert_eq!(set.get_super(0..3), None);
        assert_eq!(set.get_super(1..5), Some(1..5));
        assert_eq!(set.get_super(3..5), Some(1..5));
        assert_eq!(set.get_super(7..8), None);
        assert_eq!(set.get_super(5..10), None);
        assert_eq!(set.get_super(5..25), None);
        assert_eq!(set.get_super(10..25), Some(10..25));
        assert_eq!(set.get_super(17..25), Some(10..25));

        assert_eq!(set.get_in_range(0..25), intervals);
        assert_eq!(set.get_in_range(0..3), vec![]);
        assert_eq!(set.get_in_range(1..5), vec![1..5]);
        assert_eq!(set.get_in_range(3..5), vec![]);
        assert_eq!(set.get_in_range(7..8), vec![]);
        assert_eq!(set.get_in_range(5..10), vec![]);
        assert_eq!(set.get_in_range(5..25), vec![10..25]);
        assert_eq!(set.get_in_range(10..25), vec![10..25]);
        assert_eq!(set.get_in_range(17..25), vec![]);

        // Insert a new connecting interval and verify
        set.insert(5..10);

        let intervals: Vec<_> = set.iter().collect();
        assert_eq!(intervals, vec![1..25]);

        assert_eq!(set.get_super(0..3), None);
        assert_eq!(set.get_super(1..5), Some(1..25));
        assert_eq!(set.get_super(3..5), Some(1..25));
        assert_eq!(set.get_super(7..8), Some(1..25));
        assert_eq!(set.get_super(5..10), Some(1..25));
        assert_eq!(set.get_super(5..25), Some(1..25));
        assert_eq!(set.get_super(10..25), Some(1..25));
        assert_eq!(set.get_super(17..25), Some(1..25));

        assert_eq!(set.get_in_range(0..25), vec![1..25]);
        assert_eq!(set.get_in_range(1..25), vec![1..25]);
        assert_eq!(set.get_in_range(0..26), vec![1..25]);
        assert_eq!(set.get_in_range(1..26), vec![1..25]);
        assert_eq!(set.get_in_range(2..26), vec![]);
    }
}
