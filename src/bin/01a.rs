use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./prod_inputs/01.txt";
    let contents = fs::read_to_string(file_path)?;
    let mut current = 50;
    let mut zeroes = 0;

    for line in contents.lines() {
        let direction: char = line.chars().nth(0).unwrap();
        let number = line.chars().skip(1).collect::<String>().parse::<i16>().unwrap();

        let direction_number: i16 = match direction {
            'R' => 1,
            'L' => -1,
            _ => panic!("Invalid direction: {}", direction),
        };

        current = (((current + (direction_number * number)) % 100) + 100) % 100;

        if current == 0 {
            zeroes += 1;
        }
    }

    println!("Zeroes: {}", zeroes);

    Ok(())
}