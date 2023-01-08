use std::str::FromStr;

#[derive(Debug)]
pub struct Move {
    from: i32,
    to: i32,
    amount: i32
}

#[derive(Debug, Clone)]
struct Stack {
    number: i32,
    crates: Vec<char> 
}

impl Stack {
    pub fn collect(data: Vec<&str>) -> Vec<Stack>{
        let mut list: Vec<Stack> = Vec::new();
        let mut cloned_data = data.clone();

        let stack_guide = cloned_data.pop().expect("lets assume that aoc data is correct");
        let reversed_data: Vec<&str> = cloned_data.into_iter().rev().collect();

        for (index, stack_number) in stack_guide.chars().enumerate() {
            if !stack_number.is_whitespace() {
                let num = stack_number.to_string().parse::<i32>().unwrap();
                let get_crates: Vec<char> = reversed_data.iter()
                    .map(|x| x.chars()
                        .nth(index)
                        .unwrap())
                    .collect();

                let new_stack = Stack {
                    number: num,
                    crates: get_crates.into_iter().filter(|x| !x.is_whitespace()).collect()
                };

                list.push(new_stack);
            }
        }

        return list;
    }
}

impl FromStr for Move {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let move_list: Vec<&str> = s.split(" ").into_iter().collect();

        return Ok(Move {
            from: move_list[3].parse().unwrap(),
            to: move_list[5].parse().unwrap(),
            amount: move_list[1].parse().unwrap() 
        });
    }
}

fn main() {
    let data: Vec<&str> = include_str!("../../input/input5.prod")
        .split("\n\n")
        .collect();

    let crates_section = data[0];
    let crates: Vec<&str> = crates_section
        .lines()
        .collect();

    let game_moves_section = data[1];
    let moves_list: Vec<Move> = game_moves_section
        .lines()
        .map(|x| x.parse::<Move>().unwrap())
        .collect();

    let mut result = Stack::collect(crates);

    for move_item in moves_list {
        result = rearrange(result, move_item);
    }

    result.sort_by(|a, b| a.number.cmp(&b.number));
    println!("{:?}", result);


    // get last element from each stack
    let stuff: String = result.iter()
        .map(|x| x.crates.last().unwrap())
        .collect::<String>();

    println!("{}", stuff);
}


/*
* I don't understand the ownership nor the borrow checker
* so enjoy this abomination of a code
*/
fn rearrange(list: Vec<Stack>, mov: Move) -> Vec<Stack> {
    let mut result: Vec<Stack> = Vec::new();

    let un_touched_stacks: Vec<Stack> = list.iter() 
        .filter(|x| x.number != mov.from && x.number != mov.to)
        .cloned()
        .collect();
    
    result = result.iter().cloned().chain(un_touched_stacks.iter().cloned()).collect();

    let from_stack: &Stack = list.iter() 
        .filter(|x| x.number == mov.from)
        .collect::<Vec<&Stack>>()
        .first()
        .unwrap();

    let to_stack: &Stack = list.iter() 
        .filter(|x| x.number == mov.to)
        .collect::<Vec<&Stack>>()
        .first()
        .unwrap();

    // get crates to move
    let from_crates = from_stack.crates.clone();
    let crates_to_move: Vec<char> = from_crates[from_crates.len() - mov.amount as usize..].iter().rev().cloned().collect();

    // build new from stack
    let new_from_stack = Stack {
        number: from_stack.number,
        crates: from_crates[..from_crates.len() - mov.amount as usize].iter().cloned().collect::<Vec<char>>()
    };

    result.push(new_from_stack);

    let to_crates = to_stack.crates.clone();

    // build new to stack
    let new_to_stack = Stack {
        number: to_stack.number,
        crates: to_crates.iter().cloned().chain(crates_to_move.iter().cloned()).collect() 
    };
    
    result.push(new_to_stack);
    return result;
}
