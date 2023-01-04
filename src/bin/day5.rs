use std::str::FromStr;

#[derive(Debug)]
struct Move {
    from: i32,
    to: i32,
    amount: i32
}

#[derive(Debug)]
pub struct StackList(Vec<Stack>);

#[derive(Debug)]
struct Stack {
    number: i32,
    crates: Vec<char>
}

impl StackList {
    pub fn create_stack(&mut self, data: Vec<&str>, identity: i32, index: usize) {
        let get_crates: Vec<char> = data.iter()
            .map(|x| x.chars()
                .nth(index)
                .unwrap())
            .collect();

        let new_stack = Stack {
            number: identity,
            crates: get_crates.into_iter().filter(|x| !x.is_whitespace()).collect()
        };

        self.0.push(new_stack); // 0 reffers to Vec<Stack> field
    }
}

impl FromStr for Move {
    type Err = ();
    fn from_str(s: &str) -> Result<Move, ()> {
        let move_list: Vec<&str> = s.split(" ").into_iter().collect();

        return Ok(Move {
            from: move_list[3].parse().unwrap(),
            to: move_list[5].parse().unwrap(),
            amount: move_list[1].parse().unwrap() 
        });
    }
}

fn main() {
    let data: Vec<&str> = include_str!("../../input/input5.test")
        .split("\n\n")
        .collect();
   
    let crates_section = data[0];
    let crates: Vec<&str> = crates_section
        .lines()
        .collect();

    let stacks = build_stacks(crates);
    println!("{:?}", stacks);

    let game_moves_section = data[1];
    let moves_list: Vec<Move> = game_moves_section
        .lines()
        .map(|x| Move::from_str(x).unwrap())
        .collect();

    println!("{:?}", moves_list);
}

fn build_stacks(data: Vec<&str>) -> StackList {
    let mut list: StackList = StackList(Vec::new());

    let mut cloned_data = data.clone();
    let stack_guide = cloned_data.pop().expect("lets assume that aoc data is correct");
    let reversed_data: Vec<&str> = cloned_data.into_iter().rev().collect();

    for (index, stack_number) in stack_guide.chars().enumerate() {
        if !stack_number.is_whitespace() {
            let num = stack_number.to_string().parse::<i32>().unwrap();
            list.create_stack(reversed_data.clone(), num, index);
        }
    }
    
    return list;
}
    
fn play() {

}
