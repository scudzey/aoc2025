advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut ranges: Vec<(i64, i64)> = input
        .lines()
        .flat_map(|line| {
            line.split(',').map(|part| {
                let mut nums = part.trim().split('-');
                let start = nums.next().unwrap().parse::<i64>().unwrap();
                let end = nums.next().unwrap().parse::<i64>().unwrap();
                (start, end)
            })
        })
        .collect();
    let mut sum:u64 = 0;
    ranges.iter().for_each(|(start, end)| {
        for i in *start..=*end {
            let s = i.to_string();
            if s.len() % 2 == 0 {
                let mid = s.len() / 2;
                if &s[..mid] == &s[mid..] {
                    sum +=  i as u64;
                }
            }
        }
    });
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges: Vec<(i64, i64)> = input
        .lines()
        .flat_map(|line| {
            line.split(',').map(|part| {
                let mut nums = part.trim().split('-');
                let start = nums.next().unwrap().parse::<i64>().unwrap();
                let end = nums.next().unwrap().parse::<i64>().unwrap();
                (start, end)
            })
        })
        .collect();
    let mut sum:u64 = 0;
    ranges.iter().for_each(|(start, end)| {
        for i in *start..=*end {
            let s = i.to_string();
            let s_len = s.len();

            let mut divisors: Vec<usize> = Vec::new();
            for d in 1..=s_len/2 {
                if s_len % d == 0 {
                    divisors.push(d);
                }
            }

            for &div in &divisors {
                if div == s_len {
                    continue;
                }
                let mut all_equal = true;
                let first = &s[0..div];
                let mut index = div;
                while index < s_len {
                    let next = &s[index..index + div];
                    if first != next {
                        all_equal = false;
                        break;
                    }
                    index += div;
                }
                if all_equal {
                    sum += i as u64;
                    break;
                }
            }
        }
    });
    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
