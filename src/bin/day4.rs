fn main() {
    let sections: Vec<Vec<&str>> = include_str!("../../input/input4.prod")
        .lines()
        .map(|x| x.split(",")
            .collect::<Vec<&str>>())
        .collect();

    let mut count: i32 = 0;
    let mut part_two_count: i32 = 0;
    for section in sections {
        let assignment_one = get_assignment_range(section[0]);
        let assignment_two = get_assignment_range(section[1]);

        if are_assignments_overlapping(assignment_one.clone(), assignment_two.clone()) {
            count += 1;
        }

        if have_similar_assignments(assignment_one, assignment_two) {
            part_two_count += 1;
        }
         
    }

    println!("{}", count);
    println!("{}", part_two_count);
}

fn get_assignment_range(assignment: &str) -> Vec<i32> {
    let pairs = assignment.split("-")
        .into_iter()
        .map(|x| x.parse()
            .unwrap())
        .collect::<Vec<i32>>();

    let assignment_range: Vec<i32> = (pairs[0]..=pairs[1]).collect();
    return assignment_range;
}

fn are_assignments_overlapping(assignment_one: Vec<i32>, assignment_two: Vec<i32>) -> bool {
    let assignment_one_length = assignment_one.len();
    let assignment_two_length = assignment_two.len();

    if assignment_one_length > assignment_two_length {
        if assignment_one.first() <= assignment_two.first() && assignment_one.last() >= assignment_two.last() {
            return true;
        }
        return false; 
    }

    if assignment_one_length < assignment_two_length {
        if assignment_two.first() <= assignment_one.first() && assignment_two.last() >= assignment_one.last() {
            return true;
        }
        return false; 
    }
   
    if assignment_one_length == assignment_two_length {
        let one_sum: i32 = assignment_one.into_iter().sum();
        let two_sum: i32 = assignment_two.into_iter().sum();

        if one_sum == two_sum {
            return true;
        }
    }

    return false;
}

fn have_similar_assignments(assignment_one: Vec<i32>, assignment_two: Vec<i32>) -> bool {
    let result: Vec<i32> = assignment_one.into_iter()
        .filter(|x| assignment_two.contains(x)).collect();
    return result.len() > 0;
}
