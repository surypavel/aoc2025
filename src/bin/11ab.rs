use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;

fn count_paths(connections: &BTreeMap<&str, BTreeSet<&str>>, start: &str, end: &str) -> usize {
    let mut to_visit = BTreeMap::from([(start, 1)]);
    let mut count_out = 0;

    while !to_visit.is_empty() {
        let mut to_visit_next = BTreeMap::new();
        for (&item, &count) in to_visit.iter() {
            if item == end {
                count_out += count;
            }

            // Will not be present only for out? Idk.
            if let Some(mapref) = connections.get(&item) {
                for &item in mapref {
                    to_visit_next
                        .entry(item)
                        .and_modify(|c| *c += count)
                        .or_insert(count);
                }
            }
        }
        to_visit = to_visit_next;
    }

    return count_out;
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path: &str = "./prod_inputs/11.txt";
    let contents = fs::read_to_string(file_path)?;

    let connections: BTreeMap<&str, BTreeSet<&str>> = contents
        .split('\n')
        .map(|line| {
            let vec = line.split(": ").collect::<Vec<_>>();
            let fst = vec.get(0).unwrap();
            let snd = vec.get(1).unwrap().split(" ").collect::<BTreeSet<_>>();

            return (*fst, snd);
        })
        .collect::<BTreeMap<_, _>>();

    println!(
        "Number of paths: {}",
        count_paths(&connections, "you", "out")
    );

    println!(
        "Number of paths #2: {}",
        count_paths(&connections, "svr", "dac")
            * count_paths(&connections, "dac", "fft")
            * count_paths(&connections, "fft", "out") + 
        count_paths(&connections, "svr", "fft")
            * count_paths(&connections, "fft", "dac")
            * count_paths(&connections, "dac", "out")
    );

    Ok(())
}
