pub fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let rows: usize = lines.len();
    let cols: usize = lines.last().map(|l| l.len()).unwrap();
    // [[0; COLS]; ROWS] creates ROWS rows each initialized to an array of COLS zeros
    let mut grid: Vec<Vec<char>> = vec![vec![' '; cols]; rows];

    //INIT GRID
    for i in 0..rows {
        for j in 0..cols {
            grid[i][j] = lines[i].chars().nth(j).unwrap();
        }
    }

    let mut rolls_accessible: i32 = 0;
    //... GRID
    for i in 0..rows {
        for j in 0..cols {
            let item: char = grid[i][j];
            if item == '.' {
                continue;
            }

            if is_roll_accessible(i, j, &grid) {
                rolls_accessible = rolls_accessible + 1;
            }
        }
    }

    return rolls_accessible;
}

fn is_roll_accessible(point_x: usize, point_y: usize, grid: &Vec<Vec<char>>) -> bool {
    let mut count_of_paper_neighbors: i32 = 0;

    for i in -1isize..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let px = point_x as isize;
            let py = point_y as isize;
            let x_value_to_check = px + i;
            let y_value_to_check = py + j;

            if x_value_to_check < 0 || y_value_to_check < 0 {
                continue;
            }

            let nx = x_value_to_check as usize;
            let ny = y_value_to_check as usize;

            if nx >= grid[0].len() || ny >= grid.len() {
                continue;
            }

            if grid[nx][ny] == '@' {
                count_of_paper_neighbors = count_of_paper_neighbors + 1;
            }
        }
    }

    return count_of_paper_neighbors < 4;
}

pub fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    let rows: usize = lines.len();
    let cols: usize = lines.last().map(|l| l.len()).unwrap();
    let mut grid: Vec<Vec<char>> = vec![vec![' '; cols]; rows];

    //INIT GRID
    for i in 0..rows {
        for j in 0..cols {
            grid[i][j] = lines[i].chars().nth(j).unwrap();
        }
    }

    let mut total_rolls_removed = 0;
    let mut rolls_removed = 1;

    while rolls_removed != 0 {
        let mut rolls_accessible: i32 = 0;
        let mut rolls_to_be_removed: Vec<Coord> = Vec::new();
        //FIND ACCESSIBLE ROLLS
        for i in 0..rows {
            for j in 0..cols {
                let item: char = grid[i][j];
                if item == '.' {
                    continue;
                }

                if is_roll_accessible(i, j, &grid) {
                    rolls_to_be_removed.push(Coord::new(i, j));
                    rolls_accessible = rolls_accessible + 1;
                }
            }
        }

        rolls_removed = 0;
        //REMOVE ALL ROLLS ACCCESSIBLE
        for coord in rolls_to_be_removed {
            grid[coord.x][coord.y] = '.';
            total_rolls_removed = total_rolls_removed + 1;
            rolls_removed = rolls_removed + 1;
        }
    }

    return total_rolls_removed;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1test() {
        assert_eq!(
            part1(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            ),
            13
        );
    }

    #[test]
    fn part2test() {
        assert_eq!(
            part2(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            ),
            43
        );
    }
}
