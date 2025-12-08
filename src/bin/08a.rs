use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::collections::BTreeSet;

const NUMBER_OF_CONNECTIONS: usize = 1000;

fn distance(v1: &Vec<i64>, v2: &Vec<i64>) -> i64 {
    return v1.iter().zip(v2).map(|(&a, &b)| (a - b).abs().pow(2)).sum();
}

fn equals(v1: &Vec<i64>, v2: &Vec<i64>) -> bool {
    return v1.iter().zip(v2).all(|(&a, &b)| a == b);
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./prod_inputs/08.txt";
    let contents = fs::read_to_string(file_path)?;

    let boxes = contents
        .split('\n')
        .map(|line| {
            line.split(",")
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut min_distances: BTreeMap<i64, (Vec<i64>, Vec<i64>)> = BTreeMap::new();
    for box1 in boxes.iter() {
        for box2 in boxes.iter() {
            let dist = distance(box1, box2);

            if dist > 0 {
                min_distances.insert(dist, (box1.clone(), box2.clone()));

                if min_distances.len() > NUMBER_OF_CONNECTIONS {
                    min_distances.pop_last();
                }
            }
        }
    }

    let values = min_distances.values();
    let mut circuits: Vec<BTreeSet<Vec<i64>>> = Vec::new();

    for (e1, e2) in values {
        let mut found_circuit_index_option: Option<usize> = None;
        let mut index_to_remove_option: Option<usize> = None;

        for (index, circuit) in circuits.iter().enumerate() {
            for v in circuit.iter() {
                if equals(&e1, v) || equals(&e2, v) {
                    match found_circuit_index_option {
                        Some(_) => {
                            index_to_remove_option = Some(index);
                        }
                        None => {
                            found_circuit_index_option = Some(index);
                        }
                    }

                    break;
                }
            }
        }

        match found_circuit_index_option {
            Some(found_circuit_index) => {
                circuits[found_circuit_index].insert(e1.clone());
                circuits[found_circuit_index].insert(e2.clone());

                if let Some(index_to_remove) = index_to_remove_option {
                    let removed = circuits.swap_remove(index_to_remove);
                    for item in removed {
                        circuits[found_circuit_index].insert(item);
                    }
                }

            },
            None => {
                circuits.push(BTreeSet::from([e1.clone(), e2.clone()]));
            }
        }
    }

    let mut circuit_lengths = circuits.iter().map(|c| c.len()).collect::<Vec<_>>();

    // Just sorting everything, too lazy
    circuit_lengths.sort_by(|a, b| b.cmp(a));

    let product: usize = circuit_lengths.iter().take(3).map(|&x| x).product();

    println!("Largest circuits: {}", product);

    Ok(())
}
