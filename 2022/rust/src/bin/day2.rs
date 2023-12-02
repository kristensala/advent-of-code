use std::collections::HashMap;

fn main() {
    let strategies: Vec<&str> = include_str!("../../input/input2.prod")
        .lines()
        .collect();

    let guide = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3)
    ]);

    let guide_part2 = HashMap::from([
        ("A X", "A Z"),
        ("A Y", "A X"),
        ("A Z", "A Y"),
        ("B X", "B X"),
        ("B Y", "B Y"),
        ("B Z", "B Z"),
        ("C X", "C Y"),
        ("C Y", "C Z"),
        ("C Z", "C X")
    ]);

    let part1 = strategies.clone();

    let mut total_points: i32 = 0;
    for strat in part1 {
        total_points += guide.get(strat).unwrap();
    }

    println!("{:?}", total_points);


    let mut total_points2: i32 = 0;
    for item in strategies {
        total_points2 += guide.get(guide_part2.get(item).unwrap()).unwrap();
    }

    println!("{:?}", total_points2);
        
}
