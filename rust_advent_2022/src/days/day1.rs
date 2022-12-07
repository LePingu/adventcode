use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("/workspaces/adventcode/rust_advent_2022/src/days/test.txt")?;
    let reader = BufReader::new(file);
    let mut first: i32 = 0;
    let mut second: i32 = 0;
    let mut third: i32 = 0;
    let mut current: i32 = 0;
    // Other method
    let mut elves: Vec<i32> = vec![];
    for line in reader.lines() {
        let read_line = line?;
        if read_line.trim().is_empty() {
            if current > first {
                if first > second {
                    if second > third {
                        third = second;
                    }
                    second = first;
                }
                first = current;
            } else if current > second {
                if second > third {
                    third = second;
                }
                second = current;
            } else if current > third {
                third = current;
            }
            elves.push(current);
            current = 0;
        } else {
            current += read_line.parse::<i32>().unwrap();
        }
    }
    // Does it really take more mem ?
    elves.sort_by(|a, b| b.cmp(a));
    println!("First:{}, Second:{}, Third:{}", first, second, third);
    println!(
        "From vector tho, First:{}, Second:{}, Third:{}",
        elves[0], elves[1], elves[2]
    );
    println!("Total:{}", elves[0] + elves[1] + elves[2]);
    Ok(())
}
