use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

use lazy_static::lazy_static;

use regex::Regex;

fn extract_numbers(text: &str) -> Vec<u8> {
    lazy_static! {
        static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
    }
    NUMBER_REGEX
        .find_iter(text)
        .map(|mat| mat.as_str().parse().unwrap())
        .collect()
}

// I will be cheating and initialise the input.
// TODO: Read from file properly please !
pub fn get_9000() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("/workspaces/adventcode/rust_advent_2022/test_inputs/day5.txt")?;
    let crates: [&mut Vec<&char>; 9] = [
        &mut vec![&'T', &'P', &'Z', &'C', &'S', &'L', &'Q', &'N'],
        &mut vec![&'L', &'P', &'T', &'V', &'H', &'C', &'G'],
        &mut vec![&'D', &'C', &'Z', &'F'],
        &mut vec![&'G', &'W', &'T', &'D', &'L', &'M', &'V', &'C'],
        &mut vec![&'P', &'W', &'C'],
        &mut vec![&'P', &'F', &'J', &'D', &'C', &'T', &'S', &'Z'],
        &mut vec![&'V', &'W', &'G', &'B', &'D'],
        &mut vec![&'N', &'J', &'S', &'Q', &'H', &'W'],
        &mut vec![&'R', &'C', &'Q', &'F', &'S', &'L', &'V'],
    ];

    let test_crates: [&mut Vec<&char>; 3] = [
        &mut vec![&'Z', &'N'],
        &mut vec![&'M', &'C', &'D'],
        &mut vec![&'P'],
    ];
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let numbers_ext = extract_numbers(&line?);
        let mut counter = numbers_ext[0] as usize;
        while counter > 0 {
            let crate_m = crates[numbers_ext[1] as usize - 1].pop().unwrap();
            crates[numbers_ext[2] as usize - 1].push(crate_m);
            counter -= 1;
        }
    }
    for i in crates {
        println!("Total is {:?}", i[i.len() - 1]);
    }
    Ok(())
}

pub fn get_9001() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("/workspaces/adventcode/rust_advent_2022/test_inputs/day5.txt")?;
    let crates: [&mut Vec<&char>; 9] = [
        &mut vec![&'T', &'P', &'Z', &'C', &'S', &'L', &'Q', &'N'],
        &mut vec![&'L', &'P', &'T', &'V', &'H', &'C', &'G'],
        &mut vec![&'D', &'C', &'Z', &'F'],
        &mut vec![&'G', &'W', &'T', &'D', &'L', &'M', &'V', &'C'],
        &mut vec![&'P', &'W', &'C'],
        &mut vec![&'P', &'F', &'J', &'D', &'C', &'T', &'S', &'Z'],
        &mut vec![&'V', &'W', &'G', &'B', &'D'],
        &mut vec![&'N', &'J', &'S', &'Q', &'H', &'W'],
        &mut vec![&'R', &'C', &'Q', &'F', &'S', &'L', &'V'],
    ];

    let test_crates: [&mut Vec<&char>; 3] = [
        &mut vec![&'Z', &'N'],
        &mut vec![&'M', &'C', &'D'],
        &mut vec![&'P'],
    ];
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let numbers_ext = extract_numbers(&line?);
        let counter: usize = numbers_ext[0] as usize;
        let mut sub: Vec<&char> = crates[numbers_ext[1] as usize - 1]
            .split_off(crates[numbers_ext[1] as usize - 1].len() - counter);
        crates[numbers_ext[2] as usize - 1].append(&mut sub);
    }
    for i in crates {
        println!("Total is {:?}", i[i.len() - 1]);
    }
    Ok(())
}
