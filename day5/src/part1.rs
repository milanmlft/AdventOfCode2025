use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

struct RangeSet {
    ranges: Vec<Range<i64>>,
}

impl RangeSet {
    fn new() -> Self {
        RangeSet { ranges: Vec::new() }
    }

    fn insert(&mut self, new_range: Range<i64>) {
        if new_range.is_empty() {
            return;
        }
        self.ranges.push(new_range);
        self.merge_overlapping();
    }

    fn merge_overlapping(&mut self) {
        if self.ranges.len() <= 1 {
            return;
        }

        // Sort by start position
        self.ranges.sort_by_key(|r| r.start);

        let mut merged = Vec::new();
        let mut current = self.ranges[0].clone();

        for range in &self.ranges[1..] {
            if range.start <= current.end {
                current.end = current.end.max(range.end);
            } else {
                // No overlap, save current and start new
                merged.push(current);
                current = range.clone();
            }
        }

        merged.push(current);
        self.ranges = merged;
    }

    fn contains(&self, id: i64) -> bool {
        self.ranges.iter().any(|r| r.contains(&id))
    }
}

fn solve(filename: &str) -> i64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;

    // Collect fresh IDs first
    let mut fresh_ids = RangeSet::new();
    let mut lines_iter = reader.lines().map(|l| l.unwrap());

    for line in lines_iter.by_ref() {
        if line.is_empty() {
            // End of fresh IDs
            break;
        }
        let (range_start, range_end) = line.split_once("-").unwrap();
        let range_start = range_start.parse::<i64>().unwrap();
        let range_end = range_end.parse::<i64>().unwrap();
        fresh_ids.insert(range_start..(range_end + 1));
    }

    for line in lines_iter {
        let ingredient = line.parse::<i64>().unwrap();
        if fresh_ids.contains(ingredient) {
            result += 1;
        }
    }

    result
}

fn main() {
    println!("{}", solve("input"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day5_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 3);
    }
}
