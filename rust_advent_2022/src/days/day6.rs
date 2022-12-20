use std::{collections::HashMap, fs};

pub fn get_marker(n_marker: usize) -> usize {
    let mut letter_dict = HashMap::new();
    let _path: String = "/workspaces/adventcode/rust_advent_2022/test_inputs/day6.txt".to_owned();
    let letters_string = fs::read_to_string(_path).expect("Unable to read !");
    let letters: Vec<char> = letters_string.chars().collect();
    let mut i = 0;
    while i < letters.len() {
        let letter_byte = letters[i];
        letter_dict
            .entry(letter_byte)
            .and_modify(|lb| *lb += 1)
            .or_insert(1);
        if i > n_marker - 1 {
            let letter_byte2 = letters[i - n_marker];
            let _entry = letter_dict
                .entry(letter_byte2)
                .and_modify(|lb| *lb -= 1)
                .or_default();
            if _entry == &0 {
                letter_dict.remove(&letter_byte2);
            }
        }
        if letter_dict.len() == n_marker {
            return i + 1;
        }
        i += 1;
    }
    0
}
