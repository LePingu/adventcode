mod days;

fn main() {
    // days::day1::read_file().unwrap();
    // days::day2::solve_rock_paper_scissors().unwrap();
    let test = days::day6::get_marker(14);
    println!("{}", test);
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
