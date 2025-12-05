use std::error::Error;
use std::fs;

fn to_number(x: &str) -> i64 {
    return x.parse().unwrap();
}

fn to_tuple(x: &str) -> (i64, i64) {
    let mut split = x.split("-").map(to_number);
    return (split.next().unwrap(), split.next().unwrap());
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./prod_inputs/05.txt";
    let contents = fs::read_to_string(file_path)?;

    let parts = contents.split("\n\n").collect::<Vec<&str>>();
    let ranges: Vec<(i64, i64)> = parts[0].split("\n").map(to_tuple).collect();
    let ids = parts[1].split("\n").map(to_number);

    let fresh = ids.fold(0, |acc, curr| {
        for (r1, r2) in &ranges {
            if *r1 <= curr && curr <= *r2 {
                return acc + 1;
            }
        }

        return acc;
    });

    println!("Fresh: {}", fresh);

    Ok(())
}
