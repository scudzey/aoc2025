advent_of_code::solution!(1);


pub fn part_one(input: &str) -> Option<u64> {
    let mut start = 50;
    let mut count:u64 = 0;
    let instructions: Vec<(char, i64)> = input
        .lines()
        .map(|line| {
            let (dir, num) = line.split_at(1);
            (dir.chars().next().unwrap(), num.parse().unwrap())
        })
        .collect();
    instructions.iter().for_each(|(dir, num)| {
        match dir {
            'R' => start += num,
            'L' => start -= num,
            _ => (),
        }
        start %= 100;
        if start == 0 {
            count += 1;
        }
    });
    return Some(count);        
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut start = 50;
    let mut count:u64 = 0;
    let instructions: Vec<(char, i64)> = input
        .lines()
        .map(|line| {
            let (dir, num) = line.split_at(1);
            (dir.chars().next().unwrap(), num.parse().unwrap())
        })
        .collect();
    instructions.iter().for_each(|(dir, num)| {
        match dir {
            'R' => {
                start += num;
                if start > 100 {
                    let num_div = start /100;
                    count += num_div as u64;
                    if start % 100 == 0 {
                        count -= 1;
                    }
                }
            },
            'L' => {
                let prev_start = start;
                start -= num;
                if prev_start > 0 && start < 0 {
                    count += 1;
                }
                if start < -100 {
                    let num_div = -start /100;
                    count += num_div as u64;
                    if start % 100 == 0 {
                        count -= 1;
                    }
                }
            },
            _ => (),
        }

        start %= 100;
        if start < 0 {
            start += 100;
        }
        if start == 0 {
            count += 1;
        }
    });
    return Some(count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
