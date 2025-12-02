use std::fs;
use std::error::Error;

fn is_invalid(number: i64) -> bool {
    let number_str = number.to_string();
    let size = number_str.len();

    if (size % 2) != 0 {
        return false;
    }

    let i = size / 2;

    let substr = number_str.get(0..i).unwrap();
    let repeat = substr.repeat(size / i);

    if repeat == number_str {
        return true;
    }

    return false;
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./prod_inputs/02.txt";
    let contents = fs::read_to_string(file_path)?;
    let ranges = contents.split(",").map(|content| content.split("-"));

    let mut total: i64 = 0;

    for mut range in ranges {
        let (start, end) = (range.next().unwrap(), range.next().unwrap());
        let (start, end) = (start.parse::<i64>()?, end.parse::<i64>()?);

        // print!("{}-{}... ", start, end);

        for number in start..=end {
            if is_invalid(number) {
                // print!("{} ", number);
                total += number;
            }
        }

        // print!("\n")
    }

    println!("Total: {}", total);

    Ok(())
}