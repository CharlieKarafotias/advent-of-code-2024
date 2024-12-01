struct Counter {
    map: std::collections::HashMap<i32, i32>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            map: std::collections::HashMap::new(),
        }
    }

    fn increment(&mut self, key: i32) {
        *self.map.entry(key).or_insert(0) += 1;
    }

    fn get(&self, key: i32) -> i32 {
        *self.map.get(&key).unwrap_or(&0)
    }
}

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    // Part 1 & 2 input
    let file_path = std::path::Path::new("src/problem_1_input.txt");

    // Part 2
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

    // Part 1
    let distance = sum_of_closest_distances(v1.clone(), v2.clone());
    println!("(Part 1) The distance is: {}", distance);

    // Part 2
    let sim_score = calc_similarity_score(v1, v2);
    println!("(Part 2): The similarity score is: {}", sim_score);
}

// Used in part 1
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

// Used in part 2
pub fn calc_similarity_score(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    let mut counter = Counter::new();
    for i in v2 {
        counter.increment(i);
    }

    v1.iter().fold(0, |acc, x| acc + (x * counter.get(*x)))
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

    #[test]
    fn base_case_part_2() {
        let v1 = vec![3, 4, 2, 1, 3, 3];
        let v2 = vec![4, 3, 5, 3, 9, 3];
        let expected = 31;
        assert_eq!(calc_similarity_score(v1, v2), expected);
    }
}
