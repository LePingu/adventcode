mod days;

fn main() {
    // days::day1::read_file().unwrap();
    // days::day2::solve_rock_paper_scissors().unwrap();
    days::day4::get_overlap().unwrap();
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://adventofcode.com/2022/day/2/input")
//         .await?
//         .text()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }
