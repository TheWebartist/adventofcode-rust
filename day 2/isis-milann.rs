use std::fs;

enum RoundResult {
    Win = 6,
    Draw = 3,
    Lose = 0
}

fn main() {
    println!("########## # ############################ ##### ####################### #");
    println!("##### Advent of code 2022 - Day two - Puzzle #####");
    println!("########## # ############################ ##### ####################### #");

    let mut total_score_first_part : u32 = 0;
    let mut total_score_second_part : u32 = 0;

    let file_path = "puzzle-inputs/one";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the input file");

    let lines = contents.lines();

    for elf in lines {
        if !elf.is_empty() {
            let round_shapes : Vec<&str> = elf.split_whitespace().collect();
            let entry_shape = round_shapes[0];
            let response_shape = round_shapes[1];
            let round_result = get_round_result_from_shape_id(response_shape);
            total_score_first_part += get_score_from_shapes(entry_shape, response_shape);
            total_score_second_part += get_score_from_shape_and_round_result(entry_shape, round_result);
        }
    } 

    println!("Total score according to strategy guide (first part) : {total_score_first_part}");
    println!("Total score according to strategy guide (second part) : {total_score_second_part}");
    
    // Empty line for space between all days answers
    println!("");
}

fn get_score_from_shapes (entry_shape : &str, response_shape : &str) -> u32 {
    match entry_shape {
        "A" => {
            // Your opponent choose Rock
            match response_shape {
                "X" => return 4,
                "Y" => return 8,
                "Z" => return 3,
                _ => return 0
            }
        }
        "B" => {
            // Your opponent choose Paper
            match response_shape {
                "X" => return 1,
                "Y" => return 5,
                "Z" => return 9,
                _ => return 0
            }
        }       
        "C" => {
            // Your opponent choose Scissors
            match response_shape {
                "X" => return 7,
                "Y" => return 2,
                "Z" => return 6,
                _ => return 0
            }
        }
        _ => return 0
    }
}

fn get_round_result_from_shape_id (shape_id : &str) -> Option<RoundResult> {
    match shape_id {     
        "X" => return Some(RoundResult::Lose),
        "Y" => return Some(RoundResult::Draw),
        "Z" => return Some(RoundResult::Win),
        _ => return None
    }
}

fn get_score_from_shape_and_round_result (shape_id : &str, round_result : Option<RoundResult>) -> u32 {
    match shape_id {     
        "A" => {
            // Rock
            match round_result {
                Some(RoundResult::Lose) => return 3, // scissors,
                Some(RoundResult::Draw) => return 4, // rock
                Some(RoundResult::Win) => return 8, // paper
                _ => return 0
            }
        },
        "B" => {
            // Paper
            match round_result {
                Some(RoundResult::Lose) => return 1, // rock,
                Some(RoundResult::Draw) => return 5, // paper
                Some(RoundResult::Win) => return 9, // scissors
                _ => return 0
            }
        },
        "C" => {
            // Scissors
            match round_result {
                Some(RoundResult::Lose) => return 2, // paper,
                Some(RoundResult::Draw) => return 6, // scissors
                Some(RoundResult::Win) => return 7, // rock
                _ => return 0
            }
        },
        _ => return 0
    }
}