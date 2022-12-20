use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec;

// I'd like to replace Vec by mut [i32; 2] but wasn't able to...
// I manually map here on purpose for to try the references
fn get_common_char(line: String) -> u8 {
    let mut item_tracker: HashMap<u8, Vec<i32>> = ::std::collections::HashMap::new();
    let read_line = line;
    let byte_line = read_line.as_bytes();

    for n in 0..read_line.len() / 2 {
        let c1 = byte_line[n];
        let c2 = byte_line[read_line.len() - n - 1];
        if let std::collections::hash_map::Entry::Vacant(e) = item_tracker.entry(c1) {
            e.insert(vec![1, 0]);
        } else {
            let left = item_tracker.get_mut(&c1).unwrap();
            left[0] += 1;
            if left[0] > 0 && left[1] > 0 {
                return c1;
            }
        }
        if let std::collections::hash_map::Entry::Vacant(e) = item_tracker.entry(c2) {
            e.insert(vec![0, 1]);
        } else {
            let right = item_tracker.get_mut(&c2).unwrap();
            right[1] += 1;
            if right[0] > 0 && right[1] > 0 {
                return c2;
            }
        }
    }
    0
}

// LINQ like mapping
fn alternative_mapping(line: &str) {
    let letter_counts: HashMap<char, i32> = line.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    println!("{:?}", letter_counts);
}

// String to HashSet
fn map_letters_only(line: &str) -> HashSet<u8> {
    let letter_map: HashSet<u8> = HashSet::from_iter(line.as_bytes().iter().cloned());
    letter_map
}

fn calculate_item(item: u8) -> i32 {
    let common_value = i32::from(item);
    let difference = if item.is_ascii_lowercase() { 96 } else { 38 };
    common_value - difference
}

pub fn get_rucksacks_common() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("/workspaces/adventcode/rust_advent_2022/test_inputs/day3.txt")?;
    let reader = BufReader::new(file);
    let mut counter: i32 = 0;
    for line in reader.lines() {
        let common_char = get_common_char(line?);
        counter += calculate_item(common_char);
    }
    println!("{}", counter);
    Ok(())
}

// Trying a new method for part2
// I want to test intersection.
// TODO: Is there a way to read in chunks ?
pub fn get_elf_team_badge() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("/workspaces/adventcode/rust_advent_2022/test_inputs/day3.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader
        .read_line(&mut buf)
        .expect("reading from cursor won't fail");

    let mut counter: i32 = 0;
    let mut triplet = 2;
    let mut inter_triplet: HashSet<u8> = map_letters_only(&buf);

    for line in reader.lines() {
        if triplet > 0 {
            let temp_letters = map_letters_only(&line?);
            inter_triplet.retain(|x| temp_letters.contains(x));
            triplet -= 1;
            if triplet == 0 {
                let badge = inter_triplet.clone().into_iter().next().unwrap();
                counter += calculate_item(badge);
            }
        } else {
            inter_triplet = map_letters_only(&line?);
            triplet = 2;
        }
    }
    println!("Total is {}", counter);
    Ok(())
}
