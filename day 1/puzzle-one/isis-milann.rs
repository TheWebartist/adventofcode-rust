use std::fs;

fn main() {
    println!("########## # ############################ ##### ####################### #");
    println!("##### Advent of code 2022 - Day one - First puzzle #####");
    println!("########## # ############################ ##### ####################### #");

    let mut most_hungry_elf : i32 = 0;
    let mut current_elf_food_needs : i32 = 0;

    let file_path = "inputs/one";

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

    println!("Most hungry elf food needs : {most_hungry_elf}")
}
