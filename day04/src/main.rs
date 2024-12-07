fn main() {
    let input: Vec<Vec<char>> = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let needle_part_1: Vec<Vec<(i8, i8, char)>> = vec![
        // horizontal
        vec![(0, 0, 'X'), (0, 1, 'M'), (0, 2, 'A'), (0, 3, 'S')],
        // horizontal reverse
        vec![(0, 0, 'X'), (0, -1, 'M'), (0, -2, 'A'), (0, -3, 'S')],
        // vertical
        vec![(0, 0, 'X'), (1, 0, 'M'), (2, 0, 'A'), (3, 0, 'S')],
        // vertical reverse
        vec![(0, 0, 'X'), (-1, 0, 'M'), (-2, 0, 'A'), (-3, 0, 'S')],
        // top left -> bottom right
        vec![(0, 0, 'X'), (1, 1, 'M'), (2, 2, 'A'), (3, 3, 'S')],
        // top right -> bottom left
        vec![(0, 0, 'X'), (1, -1, 'M'), (2, -2, 'A'), (3, -3, 'S')],
        // bottom right -> top left
        vec![(0, 0, 'X'), (-1, -1, 'M'), (-2, -2, 'A'), (-3, -3, 'S')],
        // bottom left -> top right
        vec![(0, 0, 'X'), (-1, 1, 'M'), (-2, 2, 'A'), (-3, 3, 'S')],
    ];

    let part_1 = find_needle_count(&needle_part_1, &input);
    println!("{part_1}");

    let needle_part_2: Vec<Vec<(i8, i8, char)>> = vec![
        // M.S
        // .A.
        // M.S
        vec![
            (0, 0, 'M'),
            (0, 2, 'S'),
            (1, 1, 'A'),
            (2, 0, 'M'),
            (2, 2, 'S'),
        ],
        // S.S
        // .A.
        // M.M
        vec![
            (0, 0, 'S'),
            (0, 2, 'S'),
            (1, 1, 'A'),
            (2, 0, 'M'),
            (2, 2, 'M'),
        ],
        // M.M
        // .A.
        // S.S
        vec![
            (0, 0, 'M'),
            (0, 2, 'M'),
            (1, 1, 'A'),
            (2, 0, 'S'),
            (2, 2, 'S'),
        ],
        // S.M
        // .A.
        // S.M
        vec![
            (0, 0, 'S'),
            (0, 2, 'M'),
            (1, 1, 'A'),
            (2, 0, 'S'),
            (2, 2, 'M'),
        ],
    ];

    let part_2 = find_needle_count(&needle_part_2, &input);
    println!("{part_2}");
}

fn find_needle_count(needles: &Vec<Vec<(i8, i8, char)>>, haystack: &Vec<Vec<char>>) -> u16 {
    let mut result: u16 = 0;

    for i in 0..haystack.len() {
        for j in 0..haystack[i].len() {
            for direction in needles {
                if check_direction((i, j), direction, &haystack) {
                    result += 1;
                }
            }
        }
    }

    result
}

fn check_direction(
    start: (usize, usize),
    direction: &Vec<(i8, i8, char)>,
    grid: &Vec<Vec<char>>,
) -> bool {
    for delta in direction.iter() {
        if !equals_letter(start, delta, grid) {
            return false;
        }
    }

    true
}

fn equals_letter(start: (usize, usize), delta: &(i8, i8, char), grid: &Vec<Vec<char>>) -> bool {
    let row = start.0 as i16 + delta.0 as i16;
    let column = start.1 as i16 + delta.1 as i16;

    if row < 0
        || column < 0
        || row >= grid.len() as i16
        || column >= grid[row as usize].len() as i16
    {
        return false;
    }

    let char_at_delta = grid[row as usize][column as usize];

    char_at_delta == delta.2
}
