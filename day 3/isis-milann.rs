use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;

fn main() {
    println!("########## # ############################ ##### ####################### #");
    println!("##### Advent of code 2022 - Day three - Puzzle #####");
    println!("########## # ############################ ##### ####################### #");

    let mut sum_of_priorities : u32 = 0;

    let file_path = "puzzle-inputs/one";

    let priorities_ids_values_pair : HashMap<char, u8> = build_rucksack_items_priorities();

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the input file");

    let lines = contents.lines();

    for rucksack in lines {
        if !rucksack.is_empty() {
            let mut rss = String::from(rucksack);
            let separator_offset : usize = (rss.len() / 2).try_into().unwrap(); 
            // See documentation here : https://doc.rust-lang.org/std/string/struct.String.html#method.drain
            let first_compartment : String = rss.drain(..separator_offset).collect();
            let second_compartment = rss;
            sum_of_priorities += get_duplicated_items_priorities_in_rucksack_compartments(first_compartment, second_compartment, &priorities_ids_values_pair);
        }
    } 

    println!("The sum of the priorities of duplicated items in backsack's compartments : {sum_of_priorities}");
    
    // Empty line for space between all days answers
    println!("");
}

fn get_duplicated_items_priorities_in_rucksack_compartments (first_compartment : String, second_compartment : String, priorities_map : &HashMap<char, u8>) -> u32 {
    // See documentation here :
    //  - https://doc.rust-lang.org/std/string/struct.String.html#method.chars
    let mut duplicated_items : Vec<u8> = first_compartment.chars().filter(|i|second_compartment.contains(*i)).map(|i|get_priority_from_item_id(i, priorities_map)).collect();
    
    // Remove duplicate items
    duplicated_items.dedup();

    let priorities_sum = duplicated_items.iter().sum();
    return u32::from_le_bytes([priorities_sum, 0, 0, 0]);
}

fn get_priority_from_item_id (item_id : char, priorities_map : &HashMap<char, u8>) -> u8 {
    let priority = priorities_map.get(&item_id);
    match priority {
        Some(p) => return *p,
        None => {
            println!("No priority founded for item ID : {item_id}");
            return 0;
        }
    }
}

fn build_rucksack_items_priorities() -> HashMap<char, u8> {
    let mut priorities_ids_values_pair = HashMap::new();

    for i in 1..27 {
        let char_key = ((i - 1) + 'a' as u8) as char;
        let uppercase_key = char_key.to_ascii_uppercase();
        priorities_ids_values_pair.insert(char_key, i);
        priorities_ids_values_pair.insert(uppercase_key, i + 26);
    }

    return priorities_ids_values_pair;
}