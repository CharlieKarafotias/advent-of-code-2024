fn main() {
    let file_path = std::path::Path::new("day_2_input.txt");
    let mut total_safe = 0;
    for line in std::fs::read_to_string(file_path)
        .expect("Unable to read file")
        .lines()
    {
        let values: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let safe = part_1_is_safe(values);
        if safe {
            total_safe += 1;
        }
    }
    println!("{}", total_safe);
}

fn is_increasing(a: i32, b: i32) -> bool {
    b - a >= 0
}

fn safe_difference(a: i32, b: i32) -> bool {
    let diff = (a - b).abs();
    diff >= 1 && diff <= 3
}

fn part_1_is_safe(values: Vec<i32>) -> bool {
    let mut safe = true;
    let mut increasing = true;
    for i in 0..values.len() - 1 {
        let a = values[i];
        let b = values[i + 1];
        if i == 0 {
            increasing = is_increasing(a, b);
        }

        if (increasing != is_increasing(a, b)) || !safe_difference(a, b) {
            safe = false;
            break;
        }
    }
    safe
}

// TODO: Part 2: not 354 (too low)
fn part_2_is_safe_with_dampener(a: Vec<i32>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_increasing() {
        assert!(is_increasing(1, 3));
        assert!(!is_increasing(3, 1));
    }

    #[test]
    fn test_safe_difference() {
        assert!(safe_difference(1, 2));
        assert!(safe_difference(2, 1));
        assert!(!safe_difference(3, 7));
        assert!(!safe_difference(7, 1));
    }

    #[test]
    fn problem_dampener_1() {
        let a = Vec::from([1, 3, 2, 4, 5]);
        let expected = true;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_2() {
        let a = Vec::from([8, 6, 4, 4, 1]);
        let expected = true;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_3() {
        let a = Vec::from([1, 2, 7, 8, 9]);
        let expected = false;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_4() {
        // Removing 8 would make it work
        let a = Vec::from([8, 2, 3, 4, 5]);
        let expected = true;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_5() {
        // Removing 8 would make it work
        let a = Vec::from([2, 8, 3, 4, 5]);
        let expected = true;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }
}
