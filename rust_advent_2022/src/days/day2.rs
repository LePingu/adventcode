use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn init_scoring_board() -> std::collections::HashMap<&'static str, i32> {
    HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ])
}

pub fn solve_rock_paper_scissors() -> Result<i32, Box<dyn std::error::Error>> {
    let score_board = init_scoring_board();
    let file = File::open("/workspaces/adventcode/rust_advent_2022/src/days/day2_test.tx")?;
    let reader = BufReader::new(file);
    let mut total = 0;
    for line in reader.lines() {
        let line_read = line.unwrap();
        if let Some(v) = score_board.get(&line_read.as_str()) {
            total += v;
        }
    }
    println!("Total: {}", total);
    Ok(total)
}
