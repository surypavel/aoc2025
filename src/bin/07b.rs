use std::collections::BTreeMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./prod_inputs/07.txt";
    let contents = fs::read_to_string(file_path)?;

    let w = contents.split('\n').map(|line| line.chars());

    let mut beams_x: BTreeMap<usize, i64> = BTreeMap::new();

    for row in w {
        let mut beams_x_next: BTreeMap<usize, i64> = BTreeMap::new();
        let mut i: usize = 0;

        for item in row {
            if item == 'S' {
                beams_x_next.insert(i, 1);
            }

            if item == '^' {
                let current_beam = beams_x.get(&i);

                match current_beam {
                    Some(&timelines) => {
                        if i > 0 {
                            let prev_beam_timelines = beams_x_next.get(&(i - 1)).unwrap_or(&0);
                            beams_x_next.insert(i - 1, timelines + prev_beam_timelines);
                        }
                        beams_x_next.insert(i + 1, timelines);
                        beams_x.remove(&i);
                    }
                    None => {}
                }
            }

            i += 1;
        }

        for (&key, &timelines) in beams_x_next.iter() {
            // I have calculated the splitting but it still could have gotten there from the non-split path.
            beams_x.entry(key)
                .and_modify(|current_timelines| *current_timelines += timelines)
                .or_insert(timelines);
        }
    }

    let paths: i64 = beams_x.values().sum();

    println!("Paths: {}", paths);

    Ok(())
}
