use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./prod_inputs/06.txt";
    let contents = fs::read_to_string(file_path)?;

    let mut w: Vec<Vec<char>> = contents
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let operators = w.pop().unwrap();

    let mut breakpoints: Vec<usize> = operators
        .iter()
        .enumerate()
        .filter(|(_i, v)| **v == '*' || **v == '+')
        .map(|(i, _v)| i)
        .collect();

    let max_line = w.iter().map(|i| i.iter().len()).max().unwrap();
    breakpoints.push(max_line + 1);

    let mut last_breakpoint = 0;

    let mut total = 0;

    for breakpoint in breakpoints {
        if breakpoint != 0 {
            let mut ops: Vec<i64> = Vec::new();

            for index in last_breakpoint..(breakpoint - 1) {
                let mut chars: Vec<&char> = Vec::new();

                for row in w.iter() {
                    chars.push(&row[index]);
                }

                let number = chars
                    .iter()
                    .map(|&char_ref| *char_ref)
                    .collect::<String>()
                    .trim()
                    .parse::<i64>()
                    .unwrap();

                ops.push(number);
            }

            let subtotal: i64 = match operators[last_breakpoint] {
                '+' => ops.iter().sum(),
                '*' => ops.iter().product(),
                _ => panic!("Invalid operation"),
            };
            total += subtotal;
        }

        last_breakpoint = breakpoint;
    }

    println!("Math: {}", total);

    Ok(())
}
