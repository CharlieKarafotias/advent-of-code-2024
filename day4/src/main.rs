
fn main() {
    let file_path = std::path::Path::new("day_4_input.txt");
    let file = std::fs::read_to_string(file_path).expect("Unable to read file");
    let lines: Vec<&str> = file.lines().collect::<Vec<&str>>();
    let crossword: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();
    println!("Part 1 answer: {}", part_1(crossword.clone()));
}

fn part_1(crossword: Vec<Vec<char>>) -> u32 {
    let mut horizontal: Vec<String> = get_horizontal(crossword.clone());
    let mut vertical: Vec<String> = get_vertical(crossword.clone());
    let mut diagonal_up_right =
        get_diagonal(crossword.clone(), true)
            .iter()
            .map(|x| x.to_string())
            .collect();
    let mut diagonal_down_right =
        get_diagonal(crossword.clone(), false)
            .iter()
            .map(|x| x.to_string())
            .collect();

    let mut all_orientations_of_crossword: Vec<String> = Vec::new();
    all_orientations_of_crossword.append(&mut horizontal);
    all_orientations_of_crossword.append(&mut vertical);
    all_orientations_of_crossword.append(&mut diagonal_up_right);
    all_orientations_of_crossword.append(&mut diagonal_down_right);

    find_words(all_orientations_of_crossword, vec!["XMAS".to_string()])
}

fn part_2(_crossword: Vec<Vec<char>>) -> u32 {
    // only use diagonal searches now
    // need to find all mas/sam words - along with index of a
    // if you find mas/sam words - and index of a is the same in both diagonal search, then add to sum
    todo!()
}

struct Row {
    row: Vec<(char, usize, usize)>,
}
impl Row {
    fn new() -> Row {
        Row { row: Vec::new() }
    }
    fn add_letter(&mut self, letter: char, row: usize, col: usize) {
        self.row.push((letter, row, col));
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self
            .row
            .iter()
            .map(|x| x.0)
            .collect::<Vec<char>>()
            .iter()
            .collect::<String>())
    }
}

fn get_horizontal(crossword: Vec<Vec<char>>) -> Vec<String> {
    crossword
        .iter()
        .map(|x| x.iter().collect())
        .collect()
}

fn get_vertical(crossword: Vec<Vec<char>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for col_pos in 0..crossword[0].len() {
        let mut col: Vec<char> = Vec::new();
        for row_pos in 0..crossword.len() {
            col.push(crossword[row_pos][col_pos]);
        }
        result.push(col.iter().collect());
    }
    result
}

fn get_diagonal(crossword: Vec<Vec<char>>, direction_up_right: bool) -> Vec<Row> {
    let mut result: Vec<Row> = Vec::new();
    let total_diagonals = crossword.len() + crossword[0].len() - 1;
    match direction_up_right {
        true => {
            // track starting position
            let mut c_row: usize = 0;
            let mut c_col: usize = 0;

            for _diagonal in 0..total_diagonals {
                let mut row = Row::new();
                let mut r: usize = c_row;
                let mut c: usize = c_col;
                loop {
                    row.add_letter(crossword[r][c], r, c);
                    if r == 0 || c == crossword[0].len() - 1 {
                        break;
                    }
                    r -= 1;
                    c += 1;
                }
                result.push(row);
                if c_row < crossword.len() - 1 {
                    c_row += 1;
                } else {
                    c_col += 1;
                }
            }
        }
        false => {
            // down right
            // track starting position
            let mut c_row: usize = crossword.len() - 1;
            let mut c_col: usize = 0;

            for _diagonal in 0..total_diagonals {
                let mut row = Row::new();
                let mut r: usize = c_row;
                let mut c: usize = c_col;
                loop {
                    row.add_letter(crossword[r][c], r, c);
                    if r == crossword.len() - 1 || c == crossword[0].len() - 1 {
                        break;
                    }
                    r += 1;
                    c += 1;
                }
                result.push(row);
                if c_row > 0 {
                    c_row -= 1;
                } else {
                    c_col += 1;
                }
            }
        }
    }
    result
}

fn find_words(all_orientations_of_crossword: Vec<String>, words: Vec<String>) -> u32 {
    let mut sum = 0;
    for word in words {
        let word_reversed: String = word.chars().rev().collect();
        for orientation in &all_orientations_of_crossword {
            sum += orientation
                .as_bytes()
                .windows(word.len())
                .filter(|&w| w == word.as_bytes() || w == word_reversed.as_bytes())
                .count() as u32;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_horizontal() {
        let input = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
        let expected = vec!["abc".to_string(), "def".to_string()];
        assert_eq!(get_horizontal(input), expected);
    }

    #[test]
    fn test_get_vertical() {
        let input = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
        let expected = vec!["ad".to_string(), "be".to_string(), "cf".to_string()];
        assert_eq!(get_vertical(input), expected);
    }

    #[test]
    fn test_get_diagonal_down_right() {
        let input = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i']
        ];
        let expected = vec!["g".to_string(), "dh".to_string(), "aei".to_string(), "bf".to_string(), "c".to_string()];
        let result: Vec<String> = get_diagonal(input, false).iter().map(|x| x.to_string()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_diagonal_up_right() {
        let input = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i']
        ];
        let expected = vec!["a".to_string(), "db".to_string(), "gec".to_string(), "hf".to_string(), "i".to_string()];
        let result: Vec<String> = get_diagonal(input, true).iter().map(|x| x.to_string()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_words() {
        let input = vec![
            "XMASAMXAMM".to_string(),
        ];
        let expected = 2 as u32;
        assert_eq!(find_words(input, vec!["XMAS".to_string()]), expected);
    }
}