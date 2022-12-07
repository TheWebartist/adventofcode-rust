use std::fs;

fn main() {
    println!("########## # ############################ ##### ####################### #");
    println!("##### Advent of code 2022 - Day one - Puzzle #####");
    println!("########## # ############################ ##### ####################### #");

    solve_puzzle();
}

fn solve_puzzle() {
    let most_hungry_elf : u32;
    let mut current_elf_food_needs : u32 = 0;
    let mut elfs_food_needs: Vec<u32> = vec![0];

    let file_path = "puzzle-inputs/one";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the input file");

    let lines = contents.lines();

    for elf in lines {
        if elf.is_empty() {
            elfs_food_needs.push(current_elf_food_needs);
            current_elf_food_needs = 0;
        }
        else {
            current_elf_food_needs += elf.parse::<u32>().unwrap();
        }
    } 

    elfs_food_needs.sort_by(|i, m| m.partial_cmp(i).unwrap());

    most_hungry_elf = elfs_food_needs[0];
    println!("Most hungry elf food needs : {most_hungry_elf} (part one answer).");

    println!("---------- - ---------------------------- ----- ----------------------- -");
    println!("---------- HUNGRY ELFS PODIUM ----------------------- -");
    println!("---------- - ---------------------------- ----- ----------------------- -");
    let mut index = 0;
    while index < 3 {
        let food_needs = elfs_food_needs[index];
        index += 1;
        println!("Most hungry elf n°{index} food needs : {food_needs}");
    }
    println!("---------- - ---------------------------- ----- ----------------------- -");

    let hungry_elfs_podium_food_needs : u32 = elfs_food_needs.iter().take(3).sum();
    println!("Top 3 foods needs is : {hungry_elfs_podium_food_needs} (second part answer).");
    
    // Empty line for space between all days answers
    println!("");
}