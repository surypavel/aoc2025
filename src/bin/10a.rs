use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn apply_switch(memory_item: &Vec<bool>, switches: &Vec<usize>) -> Vec<bool> {
    let mut cloned = memory_item.clone();
    for switch in switches {
        cloned[*switch] = !cloned[*switch];
    }
    return cloned;

}

fn find_shortest_combination(desired_lights: Vec<bool>, switches: Vec<Vec<usize>>) -> usize {
    let mut memory: HashMap<Vec<bool>, usize> = HashMap::new();
    memory.insert((0..desired_lights.len()).map(|_| false).collect(), 0);

    while !memory.contains_key(&desired_lights) {
        let mut to_push: Vec<(Vec<bool>, usize)> = Vec::new();
        for (memory_item, count) in memory.iter() {
            for switch in switches.iter() {
                let applied = apply_switch(memory_item, switch);

                if !memory.contains_key(&applied) {
                    to_push.push((applied.clone(), count + 1));
                }
            }
        }
    
        for (memory_item, count) in to_push {
            memory.entry(memory_item).or_insert(count);
        }
    }

    return memory.get(&desired_lights).unwrap().clone();
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path: &str = "./prod_inputs/10.txt";
    let contents = fs::read_to_string(file_path)?;

    let lights_and_switches = contents
        .split('\n')
        .map(|line| {
            let mut lights: Vec<bool> = Vec::new();
            let mut switches: Vec<Vec<usize>> = Vec::new();
            let vec = line.split(" ").collect::<Vec<_>>();
            let len = vec.iter().len();

            for (index, &item) in vec.iter().enumerate() {
                if index == len - 1 {
                    // To be changed in part 2
                    continue;
                } else if index == 0 {
                    lights = item
                        .replace("[", "")
                        .replace("]", "")
                        .chars()
                        .map(|ch| ch == '#')
                        .collect();
                } else {
                    switches.push(
                        item.replace("(", "")
                            .replace(")", "")
                            .split(",")
                            .map(|i| i.parse::<usize>().unwrap())
                            .collect(),
                    )
                }
            }
            return (lights, switches);
        })
        .collect::<Vec<_>>();

    let mut total = 0;
    for (desired_lights, switches) in lights_and_switches {
        total += find_shortest_combination(desired_lights, switches);
    }

    println!("Switches to be clicked: {}", total);

    Ok(())
}
