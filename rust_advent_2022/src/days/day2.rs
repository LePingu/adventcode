use std::collections::HashMap;

pub fn init_scoring_board() -> std::collections::HashMap<&'static str, i32> {
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
