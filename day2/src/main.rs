fn main() {
    let file_path = std::path::Path::new("day_2_input.txt");
    let mut pt_1_count = 0;
    let mut pt_2_count = 0;

    for line in std::fs::read_to_string(file_path)
        .expect("Unable to read file")
        .lines()
    {
        let values: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let pt_1_safe = part_1_is_safe(values.clone());
        let pt_2_safe = part_2_is_safe_with_dampener(values);

        if pt_1_safe {
            pt_1_count += 1;
        }
        if pt_2_safe {
            pt_2_count += 1;
        }
    }
    println!("Part 1 answer: {}", pt_1_count);
    println!("Part 2 answer: {}", pt_2_count);
}

fn is_increasing(a: i32, b: i32) -> bool {
    b - a >= 0
}

fn safe_difference(a: i32, b: i32) -> bool {
    let diff = (a - b).abs();
    (1..=3).contains(&diff)
}

fn part_1_is_safe(values: Vec<i32>) -> bool {
    if values.len() < 2 {
        return true;
    }
    let mut l = 0;
    let mut r = 1;
    let increasing = is_increasing(values[l], values[r]);

    while r < values.len() {
        let safe = safe_difference(values[l], values[r]);
        if !safe || increasing != is_increasing(values[l], values[r]) {
            return false;
        }
        l += 1;
        r += 1;
    }
    true
}

fn part_2_is_safe_with_dampener(values: Vec<i32>) -> bool {
    if part_1_is_safe(values.clone()) {
        return true;
    }

    // standard rules do not apply

    // skip elem
    for i in 0..values.len() {
        let l = &values[0..i];
        let r = &values[i + 1..values.len()];
        let mut combined = Vec::new();
        combined.extend_from_slice(l);
        combined.extend_from_slice(r);

        if combined.len() > 1 && part_1_is_safe(combined) {
            return true;
        }
    }

    false
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
        let a = Vec::from([8, 2, 3, 4, 5]);
        let expected = true;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_5() {
        let a = Vec::from([2, 8, 3, 4, 5]);
        let expected = true;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_6() {
        let a = Vec::from([10, 13, 11, 12, 13]);
        let expected = true;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_7() {
        let a = Vec::from([10, 13, 11, 12, 700]);
        let expected = false;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_8() {
        let a = Vec::from([10, 200, 201, 202]);
        let expected = true;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_9() {
        let a = Vec::from([10, 20]);
        let expected = false;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_10() {
        let a = Vec::from([10, 9, 0]);
        let expected = true;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_11() {
        let a = Vec::from([10, 13, 10, 9, 8]);
        let expected = true;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_12() {
        let a = Vec::from([7, 4, 8, 9, 6]);
        let expected = false;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_13() {
        let a = Vec::from([7, 4, 3, 0, 10]);
        let expected = true;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_14() {
        let a = Vec::from([0, 7, 4, 3, 0, 10]);
        let expected = false;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn problem_dampener_15() {
        let a = Vec::from([71, 65, 59, 56, 54, 52, 51, 47]);
        let expected = false;
        assert_eq!(part_2_is_safe_with_dampener(a), expected);
    }

    #[test]
    fn pt_1_base_case() {
        let a = Vec::from([1, 2, 4, 5]);
        let expected = true;
        assert_eq!(part_1_is_safe(a), expected);
    }
}
