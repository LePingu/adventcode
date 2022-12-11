use std::fs::File;
use std::io::{BufRead, BufReader};

// I'd like to replace Vec by mut [i32; 2] but wasn't able to...
// I manually map here on purpose for to try the references
pub fn get_contained() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("/workspaces/adventcode/rust_advent_2022/src/days/day4.txt")?;
    let reader = BufReader::new(file);
    let mut count: i32 = 0;
    for line in reader.lines() {
        let pair_line = line?.clone();
        let pairs: Vec<u16> = pair_line
            .split([',', '-'])
            .flat_map(|x| x.parse())
            .collect();

        if (pairs[0] >= pairs[2] && pairs[1] <= pairs[3])
            || (pairs[2] >= pairs[0] && pairs[3] <= pairs[1])
        {
            count += 1;
            // println!("Line of content is {}", pair_line);
        }
    }
    println!("Total is {}", count);
    Ok(())
}

pub fn get_overlap() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("/workspaces/adventcode/rust_advent_2022/src/days/day4.txt")?;
    let reader = BufReader::new(file);
    let mut count: i32 = 0;
    for line in reader.lines() {
        let pair_line = line?.clone();
        let pairs: Vec<u16> = pair_line
            .split([',', '-'])
            .flat_map(|x| x.parse())
            .collect();

        if !((pairs[1] < pairs[2] && pairs[1] < pairs[3])
            || (pairs[3] < pairs[0] && pairs[3] < pairs[1]))
        {
            count += 1;
            // println!("Line of content is {}", pair_line);
        }
    }
    println!("Total is {}", count);
    Ok(())
}
