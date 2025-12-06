use std::error::Error;
use std::fs;

fn to_number(x: &str) -> i64 {
    return x.parse().unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./prod_inputs/06.txt";
    let contents = fs::read_to_string(file_path)?;

    let mut w: Vec<Vec<&str>> = contents
        .split('\n')
        .map(|line| line.split_whitespace().collect())
        .collect();

    let operators = w.pop().unwrap();

    let max = w[0].len();

    let mut total = 0;

    for n in 0..max {
        let subtotal: i64 = match operators[n] {
            "+" => w.iter().map(|x| to_number((*x)[n])).sum(),
            "*" => w.iter().map(|x| to_number((*x)[n])).product(),
            _ => panic!("Invalid operation {}", operators[n])
        };

        total += subtotal;
    }

    println!("Math: {}", total);

    Ok(())
}
