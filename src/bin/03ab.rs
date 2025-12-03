use std::fs;
use std::error::Error;


fn find_max(joltages: &str) -> (u32, usize) {
    let mut max = '0';
    let mut max_index = 0;

    for (index, joltage) in joltages.chars().enumerate() {
        if joltage > max {
            (max, max_index) = (joltage, index);
        }
    }

    return ( max.to_digit(10).unwrap(), max_index );
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./prod_inputs/03.txt";
    let contents = fs::read_to_string(file_path)?;

    let mut max_joltage: u64 = 0;
    let max_batteries = 12;

    for line in contents.lines() {
        let mut battery = 1;
        let mut index = 0;
        let mut joltage: u64 = 0;
        let len = line.len();

        while battery <= max_batteries {
            let (max, max_index) = find_max(&line[index..(len-(max_batteries - battery))]);
            battery += 1;
            index += max_index + 1;
            joltage = (joltage * 10) + u64::from(max)
        }

        max_joltage += joltage;

        println!("{}: {}", line, joltage);
    }

    println!("Joltage: {}", max_joltage);

    Ok(())
}