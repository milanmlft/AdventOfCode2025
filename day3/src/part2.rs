use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn max_joltage(bank: &str) -> u64 {
    let digits: Vec<u32> = bank.chars().flat_map(|c| c.to_digit(10)).collect();

    let mut ndigits = 12;
    let mut start_idx = 0;
    let mut result: u64 = 0;
    let base: u64 = 10;

    while ndigits > 0 {
        let mut max = 0;
        let mut offset = 0;
        let range = start_idx..digits.len() - ndigits + 1;
        for i in range {
            if digits[i] > max {
                max = digits[i];
                offset = i;
            }
        }
        start_idx = offset + 1;
        result += max as u64 * base.pow(ndigits as u32 - 1);
        ndigits -= 1;
    }

    result
}

fn solve(filename: &str) -> u64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        result += max_joltage(&line);
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
        assert_eq!(solution, 3121910778619);
    }

    #[test]
    fn day3_max_joltage_test() {
        assert_eq!(max_joltage("987654321111111"), 987654321111);
        assert_eq!(max_joltage("811111111111119"), 811111111119);
        assert_eq!(max_joltage("234234234234278"), 434234234278);
        assert_eq!(max_joltage("818181911112111"), 888911112111);
    }
}
