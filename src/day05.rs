pub fn part1(input: &str) -> i32 {
    let mut fresh_count: i32 = 0;

    let mut fresh_ingredients: Vec<Range> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let range: Range = split_into_code(line);
        fresh_ingredients.push(range);
    }

    for line in input.lines() {
        if line.contains("-") || line.is_empty() {
            continue;
        }

        let ingredient_id = line.parse::<i64>().unwrap();

        for range in &fresh_ingredients {
            if ingredient_id >= range.start && ingredient_id <= range.end {
                fresh_count = fresh_count + 1;
                break;
            }
        }
    }

    return fresh_count;
}

fn split_into_code(line: &str) -> Range {
    let parts: Vec<&str> = line.split("-").collect();

    return Range::new(
        parts
            .first()
            .map(|s| s.parse::<i64>().ok().unwrap())
            .unwrap(),
        parts
            .last()
            .map(|s| s.parse::<i64>().ok().unwrap())
            .unwrap(),
    );
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
}

pub fn part2(input: &str) -> i64 {
    let mut fresh_ingredients_ranges: Vec<Range> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let range: Range = split_into_code(line);
        fresh_ingredients_ranges.push(range);
    }

    fresh_ingredients_ranges.sort_by_key(|a| a.start);

    let mut highest_value: i64 = 0;
    let mut fresh_ingredients_count: i64 = 0;

    for range in &fresh_ingredients_ranges {
        let mut value_to_add = 0;
        if range.end < range.start {
            continue;
        }

        let mut lower_bound: i64 = range.start;

        if lower_bound <= highest_value {
            lower_bound = highest_value;
        } else {
            value_to_add = value_to_add + 1;
        }

        if range.end > highest_value {
            highest_value = range.end;
        }

        value_to_add = value_to_add + (range.end - lower_bound);

        if value_to_add < 0 {
            continue;
        }

        fresh_ingredients_count = fresh_ingredients_count + value_to_add;
    }

    return fresh_ingredients_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1test() {
        assert_eq!(
            part1(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            ),
            3
        );
    }

    #[test]
    fn part2test() {
        assert_eq!(
            part2(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            ),
            14
        );
    }

    #[test]
    fn part2test2() {
        assert_eq!(
            part2(
                "1-1
2-2
3-3
4-4
3-5

1
5
8
11
17
32"
            ),
            5
        );
    }

    #[test]
    fn part2test3() {
        assert_eq!(
            part2(
                "1-3
1-3
3-3
2-5
2-4
5-5
6-8
6-9

1
5
8
11
17
32"
            ),
            9
        );
    }
}
