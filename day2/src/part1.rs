use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// An invalid ID is any ID which is made only of some sequence of digits repeated twice
fn is_invalid(id: i64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    // Only even length IDs can be invalid
    if !len.is_multiple_of(2) {
        return false;
    }

    let parts = id_str.split_at(len / 2);
    if parts.0.parse::<usize>() == parts.1.parse::<usize>() {
        return true;
    }

    false
}

fn solve(filename: &str) -> i64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        for range in line.split(",") {
            let limits = range.split_once("-").unwrap();
            let start = limits.0.parse::<i64>().unwrap();
            let end = limits.1.parse::<i64>().unwrap();
            let ids = start..=end;

            for id in ids {
                if is_invalid(id) {
                    result += id;
                }
            }
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
    fn day2_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 1227775554);
    }

    #[test]
    fn invalid_id_test() {
        assert!(is_invalid(11));
        assert!(is_invalid(1188511885));
        assert!(is_invalid(222222));

        // Valid IDs
        assert!(!is_invalid(0));
        assert!(!is_invalid(111));
    }
}
