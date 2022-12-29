fn main() {
    let mut result: Vec<i32> = include_str!("../../input/input1.prod")
        .split("\n\n")
        .map(|x| x.lines()
            .flat_map(|y| y.parse::<i32>()).sum())
        .collect();

    result.sort_by(|a, b| b.cmp(a));

    println!("{:?}", result.first());
}
