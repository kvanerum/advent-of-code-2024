use std::collections::{BTreeSet, HashMap, HashSet};
fn main() {
    let input = read_input();

    let mut connected_sets: HashMap<usize, HashSet<BTreeSet<&str>>> = HashMap::new();
    let mut current_size = 1;

    connected_sets.insert(
        current_size,
        input
            .keys()
            .map(|s| s.to_owned())
            .map(|key| BTreeSet::from([key]))
            .collect(),
    );

    while let Some(current_sets) = connected_sets.get(&current_size).cloned() {
        // try to extend each set if it's fully connected
        for set in current_sets {
            let possible_connections = input.get(set.iter().next().unwrap()).unwrap();

            for possible_connection in possible_connections {
                let possible_connection_connections = input.get(possible_connection).unwrap();
                if set
                    .iter()
                    .all(|node| possible_connection_connections.contains(node))
                {
                    let mut next = set.clone();
                    next.insert(possible_connection);
                    connected_sets
                        .entry(current_size + 1)
                        .or_insert(HashSet::new())
                        .insert(next);
                }
            }
        }
        current_size += 1;
    }

    let part_1 = connected_sets
        .get(&3)
        .unwrap()
        .iter()
        .filter(|set| set.iter().any(|node| node.starts_with("t")))
        .count();
    println!("{part_1}");

    let part_2 = connected_sets
        .iter()
        .max_by_key(|(&key, _)| key)
        .map(|(_, set)| set.iter().next().unwrap())
        .map(|set| set.iter().map(|s| *s).collect::<Vec<&str>>().join(","))
        .unwrap();
    println!("{part_2}");
}

fn read_input() -> HashMap<&'static str, HashSet<&'static str>> {
    let mut result = HashMap::new();

    include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|s| s.split("-"))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .for_each(|(a, b)| {
            result.entry(a).or_insert(HashSet::new()).insert(b);
            result.entry(b).or_insert(HashSet::new()).insert(a);
        });

    result
}
