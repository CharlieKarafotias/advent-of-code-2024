fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    let file_path = std::path::Path::new("src/problem_1_input.txt");

    for line in std::fs::read_to_string(file_path)
        .expect("Unable to read file")
        .lines()
    {
        let mut iter = line.split_whitespace();
        let element_1 = iter.next().expect("unable to read line");
        let element_2 = iter.next().expect("unable to read line");
        v1.push(element_1.parse().expect("Failed to convert to i32"));
        v2.push(element_2.parse().expect("Failed to convert to i32"));
    }
    let distance = sum_of_closest_distances(v1, v2);

    println!("{}", distance);
}

pub fn sum_of_closest_distances(mut v1: Vec<i32>, mut v2: Vec<i32>) -> i32 {
    let mut distance_sum = 0;
    v1.sort();
    v2.sort();

    for i in 0..v1.len() {
        let v1_i = v1[i];
        let v2_i = v2[i];

        distance_sum += (v1_i - v2_i).abs();
    }
    distance_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        let v1 = vec![3, 4, 2, 1, 3, 3];
        let v2 = vec![4, 3, 5, 3, 9, 3];
        let expected = 11;

        assert_eq!(sum_of_closest_distances(v1, v2), expected);
    }
}
