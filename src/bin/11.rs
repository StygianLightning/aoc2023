const PART_1_OFFSET: usize = 1;
const PART_2_OFFSET: usize = 999999;

fn empty_row_offsets(grid: &[Vec<char>], offset: usize) -> Vec<usize> {
    let mut ret = vec![];

    let mut offset_so_far = 0;

    for row in grid.iter() {
        if row.iter().all(|c| *c == '.') {
            offset_so_far += offset;
        }
        ret.push(offset_so_far);
    }

    ret
}

fn empty_col_offsets(grid: &[Vec<char>], offset: usize) -> Vec<usize> {
    let mut ret = vec![];

    let mut offset_so_far = 0;

    for idx in 0..grid[0].len() {
        if (0..grid.len()).all(|row_idx| grid[row_idx][idx] == '.') {
            offset_so_far += offset;
        }
        ret.push(offset_so_far);
    }

    ret
}

fn main() {
    let input = std::fs::read_to_string("input/11.txt").unwrap();


    let part2 = true;

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let offset = if part2 { PART_2_OFFSET } else { PART_1_OFFSET };

    let empty_rows = empty_row_offsets(&grid, offset);
    let empty_cols = empty_col_offsets(&grid, offset);

    let galaxy_locations = find_galaxy_locations(&grid);

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
