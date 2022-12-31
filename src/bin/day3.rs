
fn main() {
    let rucksacks: Vec<&str> = include_str!("../../input/input3.prod")
        .lines()
        .collect();

    let alphabet: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut list: Vec<char> = Vec::new();

    for rucksack in rucksacks {
        let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);
        
        let mut items: Vec<char> = comp1.chars()
            .into_iter()
            .filter(|x| comp2.chars()
                .into_iter()
                .collect::<Vec<char>>()
                .contains(x))
            .collect();

        items.sort();
        items.dedup();
        list.push(items[0]);
    }

    let result: usize = list.iter_mut()
        .map(|&mut x| {
            if x.is_uppercase() {
                let lower: Vec<char> = x.to_lowercase().collect();
                return alphabet.find(lower[0]).unwrap() + 27;
            }
            return alphabet.find(x).unwrap() + 1;
        })
        .sum();

    println!("{}", result)
}
