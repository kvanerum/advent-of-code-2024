use std::collections::{HashMap, HashSet};

fn main() {
    let (frequencies, rows, columns) = read_input();

    let mut antinodes_part_1: HashSet<(usize, usize)> = HashSet::new();
    let mut antinodes_part_2: HashSet<(usize, usize)> = HashSet::new();

    frequencies.iter().for_each(|(_, antennas)| {
        for antenna1 in antennas {
            antinodes_part_2.insert(*antenna1);
            for antenna2 in antennas {
                if antenna1 != antenna2 {
                    let dx = antenna2.1 as i16 - antenna1.1 as i16;
                    let dy = antenna2.0 as i16 - antenna1.0 as i16;
                    let mut antinode_x = antenna2.1 as i16 + dx;
                    let mut antinode_y = antenna2.0 as i16 + dy;
                    
                    if is_in_bounds(antinode_y, antinode_x, rows, columns) {
                        antinodes_part_1.insert((antinode_y as usize, antinode_x as usize));
                    }
                    
                    while is_in_bounds(antinode_y, antinode_x, rows, columns) {
                        antinodes_part_2.insert((antinode_y as usize, antinode_x as usize));
                        antinode_y += dy;
                        antinode_x += dx;
                    }
                }
            }
        }
    });

    println!("{}", antinodes_part_1.len());
    println!("{}", antinodes_part_2.len());
}

fn is_in_bounds(row: i16, column: i16, num_rows: usize, num_cols: usize) -> bool {
    row >= 0 && row < num_rows as i16 && column >= 0 && column < num_cols as i16
}

fn read_input() -> (HashMap<char, HashSet<(usize, usize)>>, usize, usize) {
    let mut antennas = HashMap::new();

    let map: Vec<Vec<char>> = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    map.iter().enumerate().for_each(|(row_index, line)| {
        line.iter().enumerate().for_each(|(col_index, &ch)| {
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_insert(HashSet::new())
                    .insert((row_index, col_index));
            }
        })
    });

    (antennas, map.len(), map[0].len())
}
