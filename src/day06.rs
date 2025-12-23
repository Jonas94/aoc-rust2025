use std::ptr::null;

pub fn part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    let rows: usize = lines.len();
    let cols: usize = lines
        .last()
        .unwrap()
        .split(" ")
        .filter(|p| !p.is_empty())
        .count();

    let mut grid: Vec<Vec<&str>> = vec![vec![""; cols]; rows];

    //INIT GRID
    for i in 0..rows {
        let lineparts: Vec<&str> = lines[i].split(" ").filter(|p| !p.is_empty()).collect();
        for j in 0..cols {
            grid[i][j] = lineparts[j];
        }
    }

    let mut total_value = 0;
    for i in 0..cols {
        let action: char = grid[grid.len() - 1][i].chars().last().unwrap();
        let mut problem_list: Vec<&str> = Vec::new();

        let mut value: i64 = 0;

        if action == '*' {
            value = 1;
        } else {
            value = 0;
        }

        for j in 0..=rows - 2 {
            problem_list.push(grid[j][i]);
        }

        for problem in problem_list {
            if action == '*' {
                value = value * i64::from_str_radix(problem, 10).unwrap();
            } else {
                value = value + i64::from_str_radix(problem, 10).unwrap();
            }
        }
        total_value = total_value + value;
    }

    return total_value;
}

pub fn part2(input: &str) -> i64 {
    let mut total_value = 0;
    let lines: Vec<&str> = input.lines().collect();

    let action_line = lines.get(lines.len() - 1).unwrap();
    let mut current_index = 0;
    let mut index_of_next_action = 0;

    let mut end: bool = false;
    while !end {
        //FIND Next action position
        for i in current_index + 1..current_index + 6 {
            if i >= action_line.len() {
                index_of_next_action = action_line.len();
                end = true;
                break;
            }

            if i < action_line.len() - 1 && action_line.chars().nth(i).unwrap() != ' ' {
                index_of_next_action = i;
                break;
            }
        }

        let mut concatted_values: Vec<String> = Vec::new();
        for i in current_index..index_of_next_action {
            let mut concat_value = String::new();
            for j in 0..lines.len() - 1 {
                let char = lines.get(j).unwrap().chars().nth(i).unwrap();
                concat_value.push(char);
            }
            if (concat_value.trim().is_empty()) {
                continue;
            }
            concatted_values.push(concat_value);
        }
        //TODO: If next action missing

        let mut value: i64;

        let mut action_char = action_line.chars().nth(current_index).unwrap();
        if action_char == '*' {
            value = 1;
        } else {
            value = 0;
        }

        for val in concatted_values {
            if action_char == '*' {
                value = value * i64::from_str_radix(&val.trim(), 10).unwrap();
            } else {
                value = value + i64::from_str_radix(&val.trim(), 10).unwrap();
            }
        }
        total_value = total_value + value;

        //   println!("TOTAL VALUE NOW: {}", total_value);

        current_index = index_of_next_action;
    }

    return total_value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1test() {
        assert_eq!(
            part1(
                "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            ),
            4277556
        );
    }

    #[test]
    fn part2test() {
        assert_eq!(
            part2(
                "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            ),
            3263827
        );
    }
}
