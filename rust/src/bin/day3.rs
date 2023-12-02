fn main() {
    let rucksacks: Vec<&str> = include_str!("../../input/input3.prod")
        .lines()
        .collect();

    let alphabet: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut list: Vec<char> = Vec::new();

    for rucksack in rucksacks.clone() {
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
        .map(|&mut x| get_item_points(x, alphabet))
        .sum();

    println!("{}", result);

    println!("{}", part2(rucksacks, alphabet));
}

fn part2(data: Vec<&str>, guide: &str) -> usize {
    let mut groups: Vec<Vec<&&str>> = Vec::new();

    let mut pointer = 0;
    let chunk_size = 3;
    while pointer < data.len() {
        let group = data[pointer..pointer + chunk_size].into_iter().collect::<Vec<&&str>>();
        groups.push(group);

        pointer += chunk_size;
    }

    println!("{:?}", groups);
    
    let mut badges: Vec<char> = Vec::new();
    for group1 in groups {
        // lets assume that each group always has 3 items
        let sack1 = group1[0];
        let sack2 = group1[1];
        let sack3 = group1[2];

        for item in sack1.chars() {
            if sack2.find(item) != None && sack3.find(item) != None {
                badges.push(item);
                break;
            }
        }
    }

    println!("{:?}", badges);

    let result_part2: usize = badges.iter_mut()
        .map(|&mut x| get_item_points(x, guide))
        .sum();

    return result_part2;
}

fn get_item_points(item: char, guide: &str) -> usize {
    if item.is_uppercase() {
        let lower: Vec<char> = item.to_lowercase().collect();
        return guide.find(lower[0]).unwrap() + 27;
    }
    return guide.find(item).unwrap() + 1;
}


