use std::collections::HashSet;
use std::convert::TryInto;
use std::fs;
use std::ops::Range;

fn main() {
    println!("########## # ############################ ##### ####################### #");
    println!("##### Advent of code 2022 - Day four - Puzzle #####");
    println!("########## # ############################ ##### ####################### #");

    let mut sum_of_such_pairs : u32 = 0;
    let mut sum_of_overlapping_assignment_pairs : u32 = 0;
    
    let file_path = "puzzle-inputs/one";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the input file");

    let lines = contents.lines();

    for elf_pair in lines {
        if !elf_pair.is_empty() {
            let elfs_assignments : Vec<&str> = elf_pair.split(',').collect();

            if is_assignments_fully_contains_the_other(&elfs_assignments) {
                sum_of_such_pairs += 1;
            }

            let overlapping_areas_for_pair = get_overlapping_areas_assignments_for_elf_pair(&elfs_assignments);
            if overlapping_areas_for_pair > 0 {
                sum_of_overlapping_assignment_pairs += 1;
            }
        }
    } 

    println!("The number of assignment pairs does one range fully contain the other : {sum_of_such_pairs} [first part answer]");
    println!("The number of overlapping assignment pairs is {sum_of_overlapping_assignment_pairs} [second part answer]");
    
    // Empty line for space between all days answers
    println!("");
}

fn is_assignments_fully_contains_the_other (elfs_assignments : &Vec<&str>) -> bool {
    let areas_assigned : Vec<Vec<u32>> = elfs_assignments.iter().map(|i|get_areas_ids_for_elf_assignment(i)).collect();
    let first_elf_areas = &areas_assigned[0];
    let second_elf_areas = &areas_assigned[1];

    // The following code works just for pairs
    if first_elf_areas.iter().all(|a|second_elf_areas.contains(a)) {
        return true
    } else if second_elf_areas.iter().all(|a|first_elf_areas.contains(a)) {
        return true;
    }

    return false;
}

fn get_overlapping_areas_assignments_for_elf_pair (elfs_assignments : &Vec<&str>) -> u32 {
    let areas_assigned : Vec<Vec<u32>> = elfs_assignments.iter().map(|i|get_areas_ids_for_elf_assignment(i)).collect();
    let first_elf_areas = &areas_assigned[0];
    let second_elf_areas = &areas_assigned[1];

    let first_elf_areas_set: HashSet<u32> = first_elf_areas.iter().cloned().collect();
    let second_elf_areas_set: HashSet<u32> = second_elf_areas.iter().cloned().collect();

    let common_values: Vec<u32> = first_elf_areas_set
        .intersection(&second_elf_areas_set)
        .map(|a| *a)
        .collect();

    return common_values.len().try_into().unwrap();
}

fn get_areas_ids_for_elf_assignment (elf_assignment : &str) -> Vec<u32> {
    let area_parts : Vec<u32> = elf_assignment.split("-")
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect();

    let areas_range = Range {
        start: area_parts[0],
        end: area_parts[1] + 1,
    };
    
    return areas_range.collect();
}
