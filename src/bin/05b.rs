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
    let mut ranges = parts[0]
        .split("\n")
        .map(to_tuple)
        .collect::<Vec<(i64, i64)>>();

    ranges.sort_by_key(|(r1, _)| *r1);

    let mut fresh = 0;
    let mut prev_max = 0;

    for (r1, r2) in ranges {
        let interval_len = r2 - r1.max(prev_max + 1) + 1;

        if interval_len > 0 {
            fresh += interval_len;
            prev_max = r2;
        }
    }

    println!("Fresh: {}", fresh);

    Ok(())
}
