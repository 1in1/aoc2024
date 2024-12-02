use crate::solution;
pub struct Day4;

enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn all_directions() -> Vec<Direction> {
        vec![
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ]
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }
}

impl Day4 {
    fn is_start(input: &Vec<Vec<char>>, row: usize, col: usize, direction: &Direction) -> bool {
        let mut row_idx: i32 = row as i32;
        let mut col_idx: i32 = col as i32;
        let (row_delta, col_delta) = direction.delta();
        for c in "XMAS".chars() {
            if row_idx >= 0 && (row_idx as usize) < input.len() && col_idx >= 0 && (col_idx as usize) < input[0].len() {
                let val = input[row_idx as usize][col_idx as usize];
                if val != c {
                    return false;
                }
            } else {
                return false;
            }

            row_idx += row_delta;
            col_idx += col_delta;
        }
        return true;
    }

    fn is_x_mas_centre(input: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
        if !(row >= 1 && (row + 1) < input.len() && col >= 1 && (col + 1) < input[0].len()) {
            return false;
        }
        // Verify the central element
        if !(input[row][col] == 'A') { return false; }

        // Verify the top-left/bottom-right diagonal
        if !(
            (input[row-1][col-1] == 'M' && input[row+1][col+1] == 'S') ||
            (input[row-1][col-1] == 'S' && input[row+1][col+1] == 'M')
        ) {
            return false;
        }

        // Verify the top-right/bottom-left diagonal
        if !(
            (input[row-1][col+1] == 'M' && input[row+1][col-1] == 'S') ||
            (input[row-1][col+1] == 'S' && input[row+1][col-1] == 'M')
        ) {
            return false;
        }

        return true;
    }
}

impl solution::Solution for Day4 {
    fn solve_p2(&self, input: &str) -> String {
        let data: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        data
            .iter()
            .enumerate()
            .map(|(row_idx, row)|
                row
                    .into_iter()
                    .enumerate()
                    .filter(|(col_idx, _)|
                        Day4::is_x_mas_centre(&data, row_idx, *col_idx)
                    )
                    .count()
            )
            .sum::<usize>()
            .to_string()
    }

    fn solve_p1(&self, input: &str) -> String {
        let data: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        data
            .iter()
            .enumerate()
            .map(|(row_idx, row)|
                row
                    .into_iter()
                    .enumerate()
                    .map(|(col_idx, _)|
                        Direction::all_directions()
                            .into_iter()
                            .filter(|direction| Day4::is_start(&data, row_idx, col_idx, direction))
                            .count()
                    )
                    .sum::<usize>()
            )
            .sum::<usize>()
            .to_string()
    }
}
