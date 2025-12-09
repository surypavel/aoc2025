use std::error::Error;
use std::fs;

fn calc_area(p: (i64, i64), q: (i64, i64)) -> i64 {
    return ((p.0 - q.0).abs() + 1) * ((p.1 - q.1).abs() + 1);
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path: &str = "./prod_inputs/09.txt";
    let contents = fs::read_to_string(file_path)?;

    let red_tiles = contents
        .split('\n')
        .map(|line| {
            let vec = line.split(",")
                .map(|i| i.parse::<i64>().unwrap()).collect::<Vec<_>>();
            return (vec.get(0).unwrap().clone(), vec.get(1).unwrap().clone())
        })
        .collect::<Vec<_>>();

    let mut max = 0;
    for tile1 in red_tiles.iter() {
        for tile2 in red_tiles.iter() {
            let area = calc_area(tile1.clone(), tile2.clone());
            if area > max {
                max = area;
            }
        }
    }

    println!("Max area: {}", max);

    Ok(())
}
