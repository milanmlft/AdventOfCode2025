use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// An invalid ID is any ID which is made only of some sequence of digits repeated twice
fn is_invalid(id: i64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    if len == 1 {
        return false;
    }

    if len == 2 {
        let (first, second) = id_str.split_at(1);
        return first == second;
    }

    // Split string in all possible divisors, starting with the smallest
    let mut divisors = divisors(len as i32);
    divisors.sort();

    for d in divisors {
        let split_idx = len / d as usize;
        let (first, mut rest) = id_str.split_at(split_idx);
        while rest.len() != first.len() {
            let new_first;
            (new_first, rest) = rest.split_at(split_idx);
            if first != new_first {
                return false;
            }
        }
        if first == rest {
            return true;
        }
    }

    false
}

fn divisors(n: i32) -> Vec<i32> {
    let mut out = Vec::new();

    for i in 2..(n as f32).sqrt() as i32 + 1 {
        if n % i == 0 {
            out.push(i);
            out.push(n / i);
        }
    }
    // Include n itself as well
    out.push(n);
    out
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
    fn day2_part2_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 4174379265);
    }

    #[test]
    fn part2_invalid_id_test() {
        assert!(is_invalid(11));
        assert!(is_invalid(1212));
        assert!(is_invalid(1188511885));
        assert!(is_invalid(7777777));
        assert!(is_invalid(4444));
        assert!(is_invalid(111));
        assert!(is_invalid(1212121212));
        assert!(is_invalid(824824824));

        // Valid IDs
        assert!(!is_invalid(0));
        assert!(!is_invalid(101));
        assert!(!is_invalid(110111));
        assert!(!is_invalid(1188511888));
    }

    #[test]
    fn part2_divisors_test() {
        let mut result = divisors(12);
        result.sort();
        assert_eq!(result, vec![2, 3, 4, 6, 12])
    }
}
