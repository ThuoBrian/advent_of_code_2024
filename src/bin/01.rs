advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut first_row, mut second_row) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let number: Vec<u64> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        first_row.push(number[0]);
        second_row.push(number[1]);
    }
    first_row.sort();
    second_row.sort();

    let sum: u64 = first_row
        .iter()
        .zip(second_row.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    let (mut first_row, mut second_row) = (Vec::new(), Vec::new());

    for lines in input.lines() {
        let number: Vec<u64> = lines
            .split_whitespace()
            .filter_map(|f| f.parse().ok())
            .collect();

        first_row.push(number[0]);
        second_row.push(number[1]);
    }

    for numbers_first_row in first_row {
        sum += second_row
            .iter()
            .filter(|&num_second_row| *num_second_row == numbers_first_row)
            .sum::<u64>();
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
