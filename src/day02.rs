pub fn part1(input: &str) -> u64 {
    let mut repeated_sequences_total: u64 = 0;

    let parts: Vec<&str> = input
        .split(',')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .collect();

    for part in &parts {
        let (a, b): (u64, u64) = {
            let (left, right) = part.split_once('-').unwrap();
            let a = left.trim().parse::<u64>().unwrap();
            let b = right.trim().parse::<u64>().unwrap();
            (a, b) // no semicolon — this tuple is the block's returned value
        };

        for i in a..=b {
            if check_for_repeating_sequence(&i.to_string()) {
                //         println!("{}", &i);
                repeated_sequences_total = repeated_sequences_total + i;
            }
        }
    }
    return repeated_sequences_total;
}

fn check_for_repeating_sequence(product_id: &str) -> bool {
    let mid: usize = product_id.len() / 2;
    return &product_id[..mid] == &product_id[mid..];
}

fn find_possible_start_sequence(product_id: &str) -> Vec<&str> {
    let mut possible_start_sequences: Vec<&str> = Vec::new();

    for i in 1..=product_id.len() / 2 {
        possible_start_sequences.push(&product_id[0..i]);
    }

    return possible_start_sequences;
}

fn check_for_repeating_sequence_part2(product_id: &str) -> bool {
    let possible_start_sequences: Vec<&str> = find_possible_start_sequence(product_id);

    for sequence in possible_start_sequences {
        if product_id.len() % sequence.len() != 0 {
            continue;
        }

        let mut i: usize = sequence.len();

        let mut failed = false;
        while i < product_id.len() {
            if &product_id[i..i + sequence.len()] != sequence {
                failed = true;
                break;
            }

            i = i + sequence.len();
        }

        if !failed {
            return true;
        } else {
            return false;
        }
    }

    return false;
}

pub fn part2(input: &str) -> u64 {
    let mut repeated_sequences_total: u64 = 0;

    let parts: Vec<&str> = input
        .split(',')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .collect();

    for part in &parts {
        let (a, b): (u64, u64) = {
            let (left, right) = part.split_once('-').unwrap();
            let a = left.trim().parse::<u64>().unwrap();
            let b = right.trim().parse::<u64>().unwrap();
            (a, b)
        };

        for i in a..=b {
            if check_for_repeating_sequence_part2(&i.to_string()) {
                //  println!("{}", &i);
                repeated_sequences_total = repeated_sequences_total + i;
            }
        }
    }
    return repeated_sequences_total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1test() {
        assert_eq!(
            part1(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            1227775554
        );
    }

    #[test]
    fn part2test() {
        assert_eq!(
            part2(
                  "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            4174379265
        );
    }
}
