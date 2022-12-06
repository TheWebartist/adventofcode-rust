use std::fs;
use std::str::Lines;

fn main() {
    println!("########## # ############################ ##### ####################### #");
    println!("##### Advent of code 2022 - Day one - Puzzle #####");
    println!("########## # ############################ ##### ####################### #");

    let mut most_hungry_elf : i32 = 0;
    let mut current_elf_food_needs : i32 = 0;

    let file_path = "puzzle-inputs/one";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the input file");

    let lines = contents.lines();

    for elf in lines {
        if elf.is_empty() {
            if current_elf_food_needs > most_hungry_elf {
                most_hungry_elf = current_elf_food_needs;
            }
            current_elf_food_needs = 0;
        }
        else {
            current_elf_food_needs += elf.parse::<i32>().unwrap()
        }
    } 

    println!("Most hungry elf food needs : {most_hungry_elf}");

    get_elf_hungry_top3(contents.lines());
}

fn get_elf_hungry_top3(lines : Lines) {
    
    let mut current_elf_food_needs : i32 = 0;
    // Just want top 3
    let mut most_hungry_elfs: Vec<i32> = vec![0, 0, 0];

    for elf in lines {
        if elf.is_empty() {
            let mut elf_index : usize = 0;
            for mhe in &most_hungry_elfs {
                if mhe < &current_elf_food_needs {
                    most_hungry_elfs.insert(elf_index, current_elf_food_needs);
                    most_hungry_elfs.shrink_to_fit();
                    break;
                }
                elf_index += 1;
            }

            current_elf_food_needs = 0;
        }
        else {
            current_elf_food_needs += elf.parse::<i32>().unwrap();
        }
    } 

    let mut top_3_foods_needs : i32 = 0;
    let mut index : usize = 0;
    while index < 3 {
        let food_needs = most_hungry_elfs[index];
        println!("Most hungry elf n°{index} food needs : {food_needs}");
        top_3_foods_needs += food_needs;
        index += 1;
    }

    println!("Top 3 foods needs is : {top_3_foods_needs}")
}