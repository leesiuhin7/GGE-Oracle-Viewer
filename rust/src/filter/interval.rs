use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Clone)]
pub(crate) struct Interval {
    pub(crate) start: u32,
    pub(crate) end: u32,
}

impl Interval {
    pub(super) fn new(start: u32, end: u32) -> Result<Self, ()> {
        if start < end {
            Ok(Interval { start, end })
        } else {
            Err(())
        }
    }
}

struct IntervalByEnd(Interval);

impl PartialEq for IntervalByEnd {
    fn eq(&self, other: &Self) -> bool {
        self.0.end.eq(&other.0.end)
    }
}

impl Eq for IntervalByEnd {}

impl PartialOrd for IntervalByEnd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IntervalByEnd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.end.cmp(&other.0.end)
    }
}

pub(crate) type IntervalSet = Vec<Interval>;

fn intersect_intervals(intervals: &[&Interval]) -> Option<Interval> {
    let Interval { start, end: _ } = intervals
        .iter()
        .max_by_key(|Interval { start, end: _ }| start)?;
    let Interval { start: _, end } = intervals
        .iter()
        .min_by_key(|Interval { start: _, end }| end)?;

    Interval::new(*start, *end).ok()
}

pub(super) fn intersect_interval_sets(mut interval_sets: Vec<IntervalSet>) -> IntervalSet {
    // Setup heap
    let mut heap: BinaryHeap<(_, usize)> = BinaryHeap::with_capacity(interval_sets.len());
    // Reverse the unions so the first interval is at the end now
    for (i, set) in interval_sets.iter_mut().enumerate() {
        set.reverse();
        if let Some(interval) = set.pop() {
            heap.push((Reverse(IntervalByEnd(interval)), i));
        } else {
            return Vec::new();
        }
    }

    let mut interval_set: IntervalSet = Vec::new();
    loop {
        // Get all intervals in the heap to prepare for intersection
        let intervals = heap
            .iter()
            .map(|(Reverse(IntervalByEnd(set)), _)| set)
            .collect::<Vec<_>>();

        // Only push if an interval can be computed
        if let Some(interval) = intersect_intervals(&intervals) {
            interval_set.push(interval);
        }

        // Replace the interval that has the lowest end value
        if let Some((_, i)) = heap.pop()
            && let Some(interval) = interval_sets[i].pop()
        {
            heap.push((Reverse(IntervalByEnd(interval)), i));
        } else {
            // Exit loop if heap is empty or an interval set has been exhausted
            break;
        }
    }
    interval_set
}

fn interval_set_union(intervals: Vec<Interval>) -> IntervalSet {
    let mut result = Vec::new();

    let Some(&Interval { mut start, mut end }) = intervals.last() else {
        // Immediately return if intervals is empty
        return result;
    };
    // Skipping 1 as the first interval had already been stored
    for interval in intervals.into_iter().skip(1) {
        if end < interval.start {
            // Intervals are disjoint
            if let Ok(out_interval) = Interval::new(start, end) {
                result.push(out_interval);
            }
            Interval { start, end } = interval;
        } else {
            end = end.max(interval.end);
        }
    }
    // Push the final interval as it haven't been
    if let Ok(out_interval) = Interval::new(start, end) {
        result.push(out_interval);
    }
    result
}

pub(super) fn interval_sets_union(interval_sets: Vec<IntervalSet>) -> IntervalSet {
    let mut intervals = interval_sets.into_iter().flatten().collect::<Vec<_>>();
    intervals.sort_by_key(|interval| interval.start);
    interval_set_union(intervals)
}
