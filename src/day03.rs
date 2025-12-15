use std::cell::RefCell;
use std::char;
use std::rc::Rc;

pub fn part1(input: &str) -> i32 {
    let mut total: i32 = 0;

    for line in input.lines() {
        let first_highest_index: usize =
            find_highest_upcoming_number(0, line.chars().count() - 1, line);

        let second_highest_index: usize =
            find_highest_upcoming_number(first_highest_index + 1, line.chars().count(), line);

        let first_digit = line
            .chars()
            .nth(first_highest_index.try_into().unwrap())
            .unwrap();

        let second_digit = line
            .chars()
            .nth(second_highest_index.try_into().unwrap())
            .unwrap();

        let digits = [first_digit, second_digit];
        let digit_string: String = digits.iter().collect();
        total = total + i32::from_str_radix(&digit_string, 10).unwrap();
    }
    return total;
}

fn find_highest_upcoming_number(start: usize, end: usize, line: &str) -> usize {
    let mut highest_number: u32 = 0;

    let mut index_of_highest_number: usize = 0;
    for i in start..end {
        let digit = line
            .chars()
            .nth(i)
            .map(|c| c.to_digit(10))
            .unwrap()
            .unwrap();
        if digit > highest_number {
            highest_number = digit;
            index_of_highest_number = i;
        }
    }
    return index_of_highest_number;
}

pub fn part2(input: &str) -> i64 {
    let mut total: i64 = 0;

    for line in input.lines() {
        let highest_index_1: usize =
            find_highest_upcoming_number(0, line.chars().count() - 11, line);

        let highest_index_2: usize =
            find_highest_upcoming_number(highest_index_1 + 1, line.chars().count() - 10, line);

        let highest_index_3: usize =
            find_highest_upcoming_number(highest_index_2 + 1, line.chars().count() - 9, line);
        let highest_index_4: usize =
            find_highest_upcoming_number(highest_index_3 + 1, line.chars().count() - 8, line);
        let highest_index_5: usize =
            find_highest_upcoming_number(highest_index_4 + 1, line.chars().count() - 7, line);
        let highest_index_6: usize =
            find_highest_upcoming_number(highest_index_5 + 1, line.chars().count() - 6, line);
        let highest_index_7: usize =
            find_highest_upcoming_number(highest_index_6 + 1, line.chars().count() - 5, line);

        let highest_index_8: usize =
            find_highest_upcoming_number(highest_index_7 + 1, line.chars().count() - 4, line);

        let highest_index_9: usize =
            find_highest_upcoming_number(highest_index_8 + 1, line.chars().count() - 3, line);
        let highest_index_10: usize =
            find_highest_upcoming_number(highest_index_9 + 1, line.chars().count() - 2, line);
        let highest_index_11: usize =
            find_highest_upcoming_number(highest_index_10 + 1, line.chars().count() - 1, line);

        let highest_index_12: usize =
            find_highest_upcoming_number(highest_index_11 + 1, line.chars().count(), line);

        let digit_1 = line
            .chars()
            .nth(highest_index_1.try_into().unwrap())
            .unwrap();

        let digit_2 = line
            .chars()
            .nth(highest_index_2.try_into().unwrap())
            .unwrap();

        let digit_3 = line
            .chars()
            .nth(highest_index_3.try_into().unwrap())
            .unwrap();

        let digit_4 = line
            .chars()
            .nth(highest_index_4.try_into().unwrap())
            .unwrap();

        let digit_5 = line
            .chars()
            .nth(highest_index_5.try_into().unwrap())
            .unwrap();

        let digit_6 = line
            .chars()
            .nth(highest_index_6.try_into().unwrap())
            .unwrap();

        let digit_7 = line
            .chars()
            .nth(highest_index_7.try_into().unwrap())
            .unwrap();

        let digit_8: char = line
            .chars()
            .nth(highest_index_8.try_into().unwrap())
            .unwrap();

        let digit_9 = line
            .chars()
            .nth(highest_index_9.try_into().unwrap())
            .unwrap();

        let digit_10 = line
            .chars()
            .nth(highest_index_10.try_into().unwrap())
            .unwrap();
        let digit_11 = line
            .chars()
            .nth(highest_index_11.try_into().unwrap())
            .unwrap();

        let digit_12 = line
            .chars()
            .nth(highest_index_12.try_into().unwrap())
            .unwrap();

        let digits = [
            digit_1, digit_2, digit_3, digit_4, digit_5, digit_6, digit_7, digit_8, digit_9,
            digit_10, digit_11, digit_12,
        ];
        let digit_string: String = digits.iter().collect();
        total = total + i64::from_str_radix(&digit_string, 10).unwrap();
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1test() {
        assert_eq!(
            part1(
                "987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            357
        );
    }

    #[test]
    fn part2test() {
        assert_eq!(
            part2(
                "987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            3121910778619
        );
    }
}
