use aoc2023::util::{Grid2d, Index2d};

fn main() {
    let input_file = if let Some(file) = std::env::args().nth(1) {
        file
    } else {
        "input/17_training.txt".to_owned()
    };
    println!("using input file `{input_file}`");
    let input = std::fs::read_to_string(input_file).unwrap();
    println!("{input}");

    let part2 = if let Some(flag) = std::env::args().nth(2) {
        flag.parse().unwrap()
    } else {
        false
    };

    let mut grid = Grid2d::new(input.lines().next().unwrap().len(), input.lines().count());

    for (y, line) in input.lines().enumerate() {
        for (x, n) in line.char_indices() {
            let num = n.to_string().parse::<u32>().unwrap();
            grid[Index2d {
                x: x as _,
                y: y as _,
            }] = num;
        }
    }

    for y in 0..grid.len_y() as i32 {
        for x in 0..grid.len_x() as i32 {
            print!("{}", grid[Index2d { x, y }]);
        }
        println!();
    }
}
