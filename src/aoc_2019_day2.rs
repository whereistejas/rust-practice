use std::convert::TryFrom;

// part1 - https://adventofcode.com/2019/day/2

fn solution(mut input: Vec<i32>) -> Vec<i32> {
    for i in (0..input.len()).step_by(4) {
        let result = match input[i] {
            1 => {
                input[usize::try_from(input[i + 1]).unwrap()]
                    + input[usize::try_from(input[i + 2]).unwrap()]
            }
            2 => {
                input[usize::try_from(input[i + 1]).unwrap()]
                    * input[usize::try_from(input[i + 2]).unwrap()]
            }
            _ => break,
        };
        let res_idx = input[i + 3];
        input[usize::try_from(res_idx).unwrap()] = result;
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(solution(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    }
    #[test]
    fn sample2() {
        assert_eq!(solution(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    }
    #[test]
    fn sample3() {
        assert_eq!(
            solution(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
    }
    #[test]
    fn sample4() {
        assert_eq!(
            solution(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
    #[test]
    fn sample5() {
        assert_eq!(
            *solution(vec![
                1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 6, 19, 23, 1, 23,
                13, 27, 2, 6, 27, 31, 1, 5, 31, 35, 2, 10, 35, 39, 1, 6, 39, 43, 1, 13, 43, 47, 2,
                47, 6, 51, 1, 51, 5, 55, 1, 55, 6, 59, 2, 59, 10, 63, 1, 63, 6, 67, 2, 67, 10, 71,
                1, 71, 9, 75, 2, 75, 10, 79, 1, 79, 5, 83, 2, 10, 83, 87, 1, 87, 6, 91, 2, 9, 91,
                95, 1, 95, 5, 99, 1, 5, 99, 103, 1, 103, 10, 107, 1, 9, 107, 111, 1, 6, 111, 115,
                1, 115, 5, 119, 1, 10, 119, 123, 2, 6, 123, 127, 2, 127, 6, 131, 1, 131, 2, 135, 1,
                10, 135, 0, 99, 2, 0, 14, 0 ]).first().unwrap(),
            2782414 as i32
        );
    }
    #[test]
    fn part2_solution() {
        let mut success = false;

        let mut list: Vec<(i32, i32)>;
        list = Vec::new();
        
        for i in 0..100 {
            for j in 0..100 {
                list.push((i, j))
            }
        };

        for (noun, verb) in list.iter() {
            let first = *solution(vec![
                1, *noun, *verb, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 6, 19, 23, 1, 23,
                13, 27, 2, 6, 27, 31, 1, 5, 31, 35, 2, 10, 35, 39, 1, 6, 39, 43, 1, 13, 43, 47, 2,
                47, 6, 51, 1, 51, 5, 55, 1, 55, 6, 59, 2, 59, 10, 63, 1, 63, 6, 67, 2, 67, 10, 71,
                1, 71, 9, 75, 2, 75, 10, 79, 1, 79, 5, 83, 2, 10, 83, 87, 1, 87, 6, 91, 2, 9, 91,
                95, 1, 95, 5, 99, 1, 5, 99, 103, 1, 103, 10, 107, 1, 9, 107, 111, 1, 6, 111, 115,
                1, 115, 5, 119, 1, 10, 119, 123, 2, 6, 123, 127, 2, 127, 6, 131, 1, 131, 2, 135, 1,
                10, 135, 0, 99, 2, 0, 14, 0 ]).first().unwrap();
            if first == 19690720 {
                println!("{}, {}", noun, verb);
                success = true;
            }
        }

        assert!(success);
    }
}
