
fn main() {
    // a code is 141 and b is 142. I could use that (a = code(141) - 140) 
    let rucksacks: Vec<&str> = include_str!("../../input/input3.test")
        .lines()
        .collect();

    let mut total: i32 = 0;
    for rucksack in rucksacks {
        let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);
        
        let mut items: Vec<char> = comp1.chars()
            .into_iter()
            .filter(|x| comp2.chars()
                .into_iter()
                .collect::<Vec<char>>()
                .contains(x))
            .collect();

        items.sort(); // using dedup requires us to sort the array. Don't really need it in here bc
        // each array only contains same letters, but in other cases dedup won't do what needed.
        
        items.dedup(); // removes consecutive unique values in list except the first one
        
        // TODO: get item value and sum them

        println!("{:?}", items);
    }

    println!("{}", total);
}
