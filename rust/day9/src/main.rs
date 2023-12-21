use std::error::Error;
use day9::{read_file, part_1, part_2};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_file("./src/input.dat").unwrap();
    
    let parts: Vec<fn(&str) -> Option<isize>> = vec![part_1, part_2];
    let parts_text: Vec<&str> = vec!["Day 9, Part 1:", "Day 9, Part 2:"];

    for (part, text) in parts.iter().zip(parts_text.iter()) {
        match part(&contents) {
            Some(result)    => println!("{} {}", text, result),
            None            => println!("{} {}", text, "Not Done Yet!")
        }
    }
    
    Ok(())
}
