use std::collections::BTreeSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./prod_inputs/07.txt";
    let contents = fs::read_to_string(file_path)?;

    let w = contents
        .split('\n')
        .map(|line| line.chars());

    let mut beams_x: BTreeSet<usize> = BTreeSet::from([]);
    let mut split_count: usize = 0;

    for row in w {
        let mut beams_x_next: BTreeSet<usize> = BTreeSet::from([]);
        let mut i: usize = 0;

        for item in row {
            if item == 'S' {
                beams_x_next.insert(i);
            }

            if item == '^' && beams_x.contains(&i) {
                split_count += 1;
                beams_x_next.insert(i + 1);
                beams_x_next.insert(i - 1);
                beams_x.remove(&i);
            }

            i += 1;
        }

        for item in beams_x_next.iter() {
            beams_x.insert(*item);
        }
    }

    println!("Splittings: {}", split_count);

    Ok(())
}
