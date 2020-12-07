use std::fs;

const FILE_NAME: &str = "dataset";

fn main () {
    
    let dataset: String = fs::read_to_string(FILE_NAME).expect("Something went wrong...");

    // PART 1
    let mut floor: i16 = 0;
    let mut not_found: bool = true;
    for (i, c) in dataset.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => ()
        }
        if floor < 0 && not_found {
            println!("Basement @ {}.", i + 1);
            not_found = false;
        }
    }
    println!("{}", floor);
}