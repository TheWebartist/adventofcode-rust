use std::fs;

fn main() {
    println!("########## # ############################ ##### ####################### #");
    println!("##### Advent of code 2022 - Day two - First puzzle #####");
    println!("########## # ############################ ##### ####################### #");

    let mut total_score : u32 = 0;

    let file_path = "inputs/one";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the input file");

    let lines = contents.lines();

    for elf in lines {
        if !elf.is_empty() {
            let shapes : Vec<&str> = elf.split_whitespace().collect();
            let entry_shape = shapes[0];
            let response_shape = shapes[1];
            total_score += get_score_from_shapes(entry_shape, response_shape);
        }
    } 

    println!("Total score according to strategy guide : {total_score}");
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