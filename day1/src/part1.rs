use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn solve(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut pos = 50; // start position
    let mut result = 0; // number of times pointing at 0

    for line in reader.lines() {
        let line = line.unwrap();
        let distance = line[1..].parse::<i32>().unwrap();

        if line.starts_with("L") {
            pos -= distance;
        }
        if line.starts_with("R") {
            pos += distance;
        }

        pos %= 100; // Rescale to 0-99 limits
        if pos == 0 {
            result += 1;
        }
    }
    return result;
}

fn main() {
    println!("{}", solve("input"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day1_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 3);
    }
}
