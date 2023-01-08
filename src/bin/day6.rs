
fn main() {
    let input = include_str!("../../input/input6.prod");
    let arr: Vec<char> = input.chars().collect();

    let mut res = 0;
    for (i, _) in arr.iter().enumerate() {
        let end = i + 4;
        let result: Vec<u32> = arr[i..end].iter() // i am missing the out of bounds check, but got
            // the right anwser anyway
            .map(|x| {
                return *x as u32;
            })
            .collect();
        
        if !has_duplicate(result) {
            res = end;
            break;
        }
    }

    println!("{}", res);
}

fn has_duplicate(mut arr: Vec<u32>) -> bool {
    arr.sort_by(|a, b| a.cmp(b));

    let mut result = false;
    for (i, letter) in arr.iter().enumerate() {
        if i < arr.len() - 1 {
            let xor = letter ^ arr[i + 1];
            if xor == 0 {
                result = true;
                break;
            }
        }
    }

    return result;
}
