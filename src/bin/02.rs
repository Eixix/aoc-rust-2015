advent_of_code::solution!(2);

struct Box {
    length: usize,
    width: usize,
    height: usize,
}

impl Box {
    fn sorted_sides(&self) -> Vec<usize> {
        let mut vector = vec![self.length, self.width, self.height];
        vector.sort();
        vector
    }

    fn volume(&self) -> usize {
        self.length * self.width * self.height
    }
}

fn parse_boxes(s: &str) -> Vec<Box> {
    let line_iterator = s.lines();
    let mut boxes: Vec<Box> = Vec::new();
    for line in line_iterator {
        let attributes = line.split('x').collect::<Vec<&str>>().iter().map(|x| { x.parse::<usize>().unwrap() }).collect::<Vec<usize>>();
        match attributes[..] {
            [l, w, h] => boxes.push(Box { length: l, width: w, height: h }),
            _ => {}
        }
    }

    boxes
}

pub fn part_one(input: &str) -> Option<u32> {
    let boxes = parse_boxes(input);
    let mut sum = 0;
    for b in boxes.iter()
    {
        let total_area = 2 * b.length * b.width + 2 * b.width * b.height + 2 * b.height * b.length;
        let smallest_area = b.sorted_sides()[0] * b.sorted_sides()[1];
        sum += total_area + smallest_area;
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let boxes = parse_boxes(input);
    let mut sum = 0;
    for b in boxes.iter() {
        sum += b.sorted_sides()[0] * 2 + b.sorted_sides()[1] * 2 + b.volume();
    }

    Some(sum as u32)
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
