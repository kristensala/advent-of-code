fn main() {
    let mut result: Vec<i32> = include_str!("../../input/input1.prod")
        .split("\n\n")
        .map(|x| x.lines()
            .flat_map(|y| y.parse::<i32>()).sum())
        .collect();
    
    result.sort_by(|a, b| b.cmp(a));
    let part2: i32 = result.into_iter().take(3).collect::<Vec<i32>>().iter().sum();

    println!("{:?}", part2);
}
