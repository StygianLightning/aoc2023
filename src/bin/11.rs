fn empty_row_offsets(grid: &[Vec<char>]) -> Vec<usize> {
    let mut ret = vec![];

    let mut offset = 0;

    for row in grid.iter() {
        if row.iter().all(|c| *c == '.') {
            offset += 1;
        }
        ret.push(offset);
    }

    ret
}

fn empty_col_offsets(grid: &[Vec<char>]) -> Vec<usize> {
    let mut ret = vec![];

    let mut offset = 0;

    for idx in 0..grid[0].len() {
        if (0..grid.len()).all(|row_idx| grid[row_idx][idx] == '.') {
            offset += 1;
        }
        ret.push(offset);
    }

    ret
}

fn main() {
    let input = std::fs::read_to_string("input/11_training.txt").unwrap();

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let empty_rows = empty_row_offsets(&grid);
    let empty_cols = empty_col_offsets(&grid);

    println!("empty row offsets: {empty_rows:?}");
    println!("empty col offsets: {empty_cols:?}");

    let galaxy_locations = find_galaxy_locations(&grid);

    println!("galaxy locations: {galaxy_locations:?}");

    let total = all_pairs_shortest_path_sum(galaxy_locations, empty_rows, empty_cols);

    println!("total: {total}");
}

fn all_pairs_shortest_path_sum(
    galaxy_locations: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
) -> u64 {
    let mut total = 0;
    for (i, (a, b)) in galaxy_locations.iter().cloned().enumerate() {
        for (c, d) in galaxy_locations[i + 1..].iter().cloned() {
            let adjusted_a = a + empty_rows[a];
            let adjusted_b = b + empty_cols[b];
            let adjusted_c = c + empty_rows[c];
            let adjusted_d = d + empty_cols[d];

            let row_diff = (adjusted_a as i32 - adjusted_c as i32).unsigned_abs();
            let col_diff = (adjusted_b as i32 - adjusted_d as i32).unsigned_abs();

            total += (row_diff + col_diff) as u64;
        }
    }

    total
}

fn find_galaxy_locations(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut ret = vec![];

    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                ret.push((i, j));
            }
        }
    }

    ret
}
