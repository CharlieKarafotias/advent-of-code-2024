use regex::Regex;

fn main() {
    let file_path = std::path::Path::new("day_3_input.txt");
    let file = std::fs::read_to_string(file_path).expect("Unable to read file");
    // NOTE: not clear that the input is one whole line. It should've been mentioned in the desc.
    let lines: String = file.lines().collect::<Vec<&str>>().join("");

    println!("Part 1 answer: {}", part_1(lines.clone()));
    println!("Part 2 answer: {}", part_2(lines.clone()));
}

fn part_1(input: String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&input).fold(0, |acc, m| {
        let caps = m;
        let a: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let b: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        acc + (a * b)
    })
}

fn part_2(input: String) -> i32 {
    let mut sum = 0;
    let mut do_op = true;
    let mut rest_of_line = input.clone();

    while rest_of_line.len() > 0 {
        let mut query = "don't()".to_string();
        if do_op {
            query = "don't()".to_string();
        } else {
            query = "do()".to_string();
        }

        let split = rest_of_line.splitn(2, &query).collect::<Vec<&str>>();
        if split.len() == 0 {
            break;
        }
        let left = split[0];
        let right = split.get(1).unwrap_or(&"");

        if do_op {
            sum += part_1(left.to_string());
        }

        do_op = !do_op;
        rest_of_line = right.to_string();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let line: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_1(line.to_string()), 161);
    }

    #[test]
    fn test_part_2() {
        let line: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_2(line.to_string()), 48);
    }

    #[test]
    fn test_part_2_b() {
        let line: &str = "don't()mul(2,4)&mul[3,7]!^don't()_mul(5,5)do()+mul(32,64](mul(11,8)undo()?mul(8,5))do()";
        assert_eq!(part_2(line.to_string()), 128);
    }
    #[test]
    fn test_part_2_c() {
        let line: &str = "don't()mul(2,4)&mul[3,7]!^_mul(5,5)+mul(32,64](mul(11,8)un?mul(8,5))do()";
        assert_eq!(part_2(line.to_string()), 0);
    }
}
