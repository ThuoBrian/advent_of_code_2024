advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    // let test_input = "7 6 4 2 1 \n 1 2 7 8 9 \n 9 7 6 2 1 \n 1 3 2 4 5 \n 8 6 4 4 1 \n 1 3 6 7 9 ";

    let mut safe_report = 0;

    for lines_in_test_input in input.lines() {
        let report_table: Vec<u32> = lines_in_test_input
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        let increasing = report_table
            .windows(2)
            .all(|x| (x[0] >= x[1]) && (x[0] - x[1] >= 1) && (x[0] - x[1] <= 3));
        let decreasing = report_table
            .windows(2)
            .all(|x| (x[0] <= x[1]) && (x[1] - x[0] >= 1) && (x[1] - x[0] <= 3));

        if decreasing || increasing {
            safe_report += 1;
        }
    }

    Some(safe_report)
}

pub fn part_two(input: &str) -> Option<u64> {
    let test_input = "7 6 4 2 1 \n 1 2 7 8 9 \n 9 7 6 2 1 \n 1 3 2 4 5 \n 8 6 4 4 1 \n 1 3 6 7 9 ";

    let mut safe_report = 0;

    for lines in test_input.lines() {
        let array_numbers: Vec<u32> = lines
            .split_whitespace()
            .filter_map(|f| f.parse().ok())
            .collect();
        let increasing_safe_rows = array_numbers
            .windows(2)
            .all(|x| (x[0] >= x[1]) && (x[0] - x[1] >= 1) && (x[0] - x[1] <= 3));
        let decreasing_safe_rows = array_numbers
            .windows(2)
            .all(|x| (x[0] <= x[1]) && (x[1] - x[0] >= 1) && (x[1] - x[0] <= 3));

      
    }
    Some(safe_report)
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
