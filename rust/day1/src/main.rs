use std::error::Error;
use day1::{read_file, part_1, part_2};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_file("./input.dat").unwrap();

    println!("Answer to part 1: {}", &part_1(&contents));
    println!("Answer to part 2: {}", &part_2(&contents));
    
    Ok(())
}
