fn main() {
    let (keys, locks) = read_input();

    let mut count = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if fits(lock, key)  {
                count += 1;
            }
        }
    }

    println!("{count}");
}

fn fits(lock: &Vec<u8>, key: &Vec<u8>) -> bool {
    for i in 0..key.len() {
        if key.get(i).unwrap() + lock.get(i).unwrap() > 5 {
            return false;
        }
    }
    
    true
}

fn read_input() -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    include_str!("../resources/input.txt")
        .trim_end()
        .split("\n\n")
        .for_each(|element| {
            let chars: Vec<Vec<char>> =
                element.lines().map(|line| line.chars().collect()).collect();

            if *chars.get(0).unwrap().get(0).unwrap() == '#' {
                let mut lock = Vec::new();

                for column in 0..chars.get(0).unwrap().len() {
                    let mut count = 0;

                    while *chars.get(1 + count).unwrap().get(column).unwrap() == '#' {
                        count += 1;
                    }

                    lock.push(count as u8);
                }

                locks.push(lock);
            } else {
                let mut key = Vec::new();

                for column in 0..chars.get(0).unwrap().len() {
                    let mut count = 0;

                    while *chars
                        .get(chars.len() - 2 - count)
                        .unwrap()
                        .get(column)
                        .unwrap()
                        == '#'
                    {
                        count += 1;
                    }

                    key.push(count as u8);
                }

                keys.push(key);
            }
        });

    (keys, locks)
}
