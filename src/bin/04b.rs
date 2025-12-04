use std::error::Error;
use std::fs;

const COORDS: [(usize, usize); 8] = [
    (2, 2),
    (0, 0),
    (2, 0),
    (0, 2),
    (2, 1),
    (1, 2),
    (0, 1),
    (1, 0),
];

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./prod_inputs/04.txt";
    let contents = fs::read_to_string(file_path)?;

    let lines = contents.lines().collect::<Vec<&str>>();
    let width = lines.len();
    let height = lines.first().unwrap().len();

    let mut rocks = vec![vec![false; width + 2]; height + 2];

    // Convert lines to rocks map (probably can be done easier)
    let mut x = 1;
    for line in lines {
        let mut y = 1;
        for char in line.chars() {
            if char == '@' {
                rocks[x][y] = true;
            }

            y += 1;
        }
        x += 1;
    }

    let mut rolls_total = 0;

    loop {
        let mut state = vec![vec![0; width + 2]; height + 2];

        let mut x = 1;
        while x <= width {
            let mut y = 1;
            while y <= height {
                if rocks[x][y] == true {
                    for (dx, dy) in COORDS {
                        let xx = x + dx - 1;
                        let yy = y + dy - 1;
                        state[xx][yy] += 1;
                    }
                }

                y += 1;
            }
            x += 1;
        }

        let mut rolls = 0;
        let mut i = 1;

        while i <= width {
            let mut j = 1;
            while j <= height {
                if state[i][j] < 4 && rocks[i][j] == true {
                    // println!("{} {}", i, j);
                    rolls += 1;
                    rocks[i][j] = false;
                }
                j += 1;
            }
            i += 1;
        }

        rolls_total += rolls;

        if rolls == 0 {
            break;
        }
    }

    println!("Rolls: {}", rolls_total);

    Ok(())
}
