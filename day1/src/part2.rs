use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn solve(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut pos: i32 = 50; // start position
    let mut result = 0; // number of times pointing at 0

    for line in reader.lines() {
        let line = line.unwrap();
        let distance = line[1..].parse::<i32>().unwrap();

        if line.starts_with("L") {
            if pos == 0 {
                pos += 100;
            }
            pos -= distance;
        } else {
            pos += distance;
        }

        // Scale pos back to 0-99 limits and count times 0 is passed
        while pos < 0 {
            pos += 100;
            result += 1;
        }
        while pos > 100 {
            pos -= 100;
            result += 1;
        }
        if pos == 100 {
            pos = 0;
            result += 1;
        } else if pos == 0 {
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
    fn day1_part2_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 6);
    }
}
