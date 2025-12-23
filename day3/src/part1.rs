use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn max_joltage(bank: &[u32]) -> u32 {
    let first_idx = bank[..bank.len() - 1]
        .iter()
        .enumerate()
        .max_by_key(|&(_idx, value)| value)
        .map(|(idx, _)| idx)
        .unwrap();

    let first = bank[first_idx];
    let mut second = *bank[first_idx + 1..].iter().max().unwrap();

    // max_by_key() returns the last element in case of ties
    // so we need to check if first is duplicated to resolve this
    let duplicate_max = bank[..first_idx].contains(&first);
    if second < first && duplicate_max {
        second = first;
    }

    bank[first_idx] * 10 + second
}

fn solve(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let digits: Vec<u32> = line.chars().flat_map(|c| c.to_digit(10)).collect();
        result += max_joltage(&digits);
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
    fn day3_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 357);
    }

    #[test]
    fn day3_max_joltage_test() {
        assert_eq!(max_joltage(&[9, 8, 7, 6, 1]), 98);
        assert_eq!(max_joltage(&[5, 2, 5, 1, 5]), 55);
        assert_eq!(max_joltage(&[1, 1, 1, 8, 9]), 89);
        assert_eq!(max_joltage(&[5, 2, 5, 1, 9]), 59);
    }
}
