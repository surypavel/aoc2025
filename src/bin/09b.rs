use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn in_interval(p: i64, e: (i64, i64)) -> bool {
    return (e.1 <= p && p <= e.0) || (e.1 >= p && p >= e.0);
}

fn calc_area(p: (i64, i64), q: (i64, i64)) -> i64 {
    return ((p.0 - q.0).abs() + 1) * ((p.1 - q.1).abs() + 1);
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path: &str = "./prod_inputs/09.txt";
    let contents = fs::read_to_string(file_path)?;

    let mut corners: HashMap<(i64, i64), bool> = HashMap::new();
    let red_tiles = contents
        .split('\n')
        .map(|line| {
            let vec = line
                .split(",")
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            return (vec.get(0).unwrap().clone(), vec.get(1).unwrap().clone());
        })
        .collect::<Vec<_>>();

    let max_x = red_tiles.iter().map(|(x, _)| x).max().unwrap();

    // Add corners
    for tile1 in red_tiles.iter() {
        for tile2 in red_tiles.iter() {
            let corner1 = ((*tile1).0, (*tile2).1);
            let corner2 = ((*tile2).0, (*tile1).1);

            corners.insert(corner1, false);
            corners.insert(corner2, false);
        }
    }

    // Create horizontal edges
    let mut edges_horizontal: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut edges_vertical: HashMap<i64, Vec<i64>> = HashMap::new();

    for &v in red_tiles.iter() {
        edges_horizontal.entry(v.1).or_default().push(v.0);
        edges_vertical.entry(v.0).or_default().push(v.1);
    }

    for (&height, vector) in edges_horizontal.iter() {
        let e1 = vector.get(0).unwrap().clone();
        let e2 = vector.get(1).unwrap().clone();
        let edge = (
            if e1 == *max_x { e1 } else { e1 - 1 },
            if e2 == *max_x { e2 } else { e2 - 1 },
        );

        for (corner, is_inside) in corners.iter_mut() {
            if corner.1 > height {
                if in_interval(corner.0, edge) {
                    *is_inside = !*is_inside;
                }
            }
        }
    }

    for (&height, vector) in edges_horizontal.iter() {
        let edge = (
            vector.get(0).unwrap().clone(),
            vector.get(1).unwrap().clone(),
        );

        for (corner, is_inside) in corners.iter_mut() {
            if corner.1 == height {
                if in_interval(corner.0, edge) {
                    *is_inside = true;
                }
            }
        }
    }

    let mut max = 0;
    for tile1 in red_tiles.iter() {
        for tile2 in red_tiles.iter() {
            let area = calc_area(*tile1, *tile2);

            if area > max {
                if corners
                    .iter()
                    .filter(|((a1, a2), _)| {
                        in_interval(*a1, ((*tile1).0, (*tile2).0))
                            && in_interval(*a2, ((*tile1).1, (*tile2).1))
                    })
                    .all(|(_, &is_inside)| is_inside)
                {
                    max = area;
                }
            }
        }
    }

    println!("Max area: {}", max);

    Ok(())
}
