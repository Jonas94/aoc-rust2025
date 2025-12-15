use std::collections::LinkedList;

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
        let mut index_list: LinkedList<usize> = LinkedList::new();

        let mut highest_index = find_highest_upcoming_number(0, line.chars().count() - 11, line);
        index_list.push_back(highest_index);

        for i in 0..11 {
            highest_index = find_highest_upcoming_number(
                highest_index + 1,
                line.chars().count() - (11 - i - 1),
                line,
            );
            index_list.push_back(highest_index);
        }

        let mut digit_string = String::new();
        for i in 0..12 {
            digit_string.push(line.chars().nth(index_list.pop_front().unwrap()).unwrap());
        }
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
