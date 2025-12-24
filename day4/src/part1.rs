use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<char>>,
    nrow: usize,
    ncol: usize,
}

impl Grid {
    fn count_neighbours(&self, x: usize, y: usize) -> u32 {
        let mut n = 0;

        let nrow = self.nrow as isize;
        let ncol = self.ncol as isize;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let i = x as isize + dx;
                let j = y as isize + dy;

                if i >= 0
                    && i < nrow
                    && j >= 0
                    && j < ncol
                    && self.cells[i as usize][j as usize] == '@'
                {
                    n += 1;
                }
            }
        }
        n
    }
}

fn solve(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Construct grid
    let mut grid = Grid {
        cells: Vec::new(),
        nrow: 0,
        ncol: 0,
    };

    for line in reader.lines() {
        grid.cells.push(line.unwrap().chars().collect());
        grid.nrow += 1;
    }
    grid.ncol = grid.cells[0].len();

    let mut result = 0;
    for (i, _) in grid.cells.iter().enumerate() {
        for (j, _) in grid.cells[i].iter().enumerate() {
            if grid.cells[i][j] == '@' && grid.count_neighbours(i, j) < 4 {
                result += 1;
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
    fn day4_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 13);
    }
}
