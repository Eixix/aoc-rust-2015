advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let opening_brackets: u32 = input.matches('(').count() as u32;
    let closing_brackets: u32 = input.matches(')').count() as u32;
    Some(opening_brackets - closing_brackets)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut position: u32 = 1;
    let mut sum: i32 = 0;
    for char in input.chars() {
        match char {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => {}
        }
        if sum == -1 {
            return Some(position);
        }
        position += 1;
    }
    None
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
