use std::vec;

fn visible_trees(grid: &Vec<Vec<u32>>) -> (u32, Vec<(usize, usize)>) {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    let mut positions = vec![];

    for row in 0..rows {
        for col in 0..cols {
            let tree = grid[row][col];

            let left_visible = (0..col).all(|c| grid[row][c] < tree);
            let right_visible = (col + 1..cols).all(|c| grid[row][c] < tree);
            let up_visible = (0..row).all(|r| grid[r][col] < tree);
            let down_visible = (row + 1..rows).all(|r| grid[r][col] < tree);

            if left_visible || right_visible || up_visible || down_visible {
                count += 1;
                positions.push((row, col));
            }
        }
    }

    (count, positions)
}

fn main() {
    // read input from a file
    let input = std::fs::read_to_string("input.txt").unwrap();

    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let (count, positions) = visible_trees(&grid);

    println!("Number of visible trees: {}", count);

    let max_distance = positions
        .iter()
        .map(|(row, col)| view_distance(&grid, *row, *col))
        .max();

    println!("Max distance: {}", max_distance.unwrap());
}

fn view_distance(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let tree = grid[row][col];

    // left
    let left_distance = (0..col)
        .rev()
        .map(|c| grid[row][c])
        .take_while(|&cell| cell < tree)
        .count();

    // right
    let right_distance = (col + 1..cols)
        .map(|c| grid[row][c])
        .take_while(|&cell| cell < tree)
        .count();

    // up
    let up_distance = (0..row)
        .rev()
        .map(|r| grid[r][col])
        .take_while(|&cell| cell < tree)
        .count();

    // down
    let down_distance = if row == rows - 1 {
        0
    } else {
        (row + 1..rows)
            .map(|r| grid[r][col])
            .take_while(|&cell| cell < tree)
            .count() + 1
    };

    // return the multiplication of the distances
    left_distance * right_distance * up_distance * down_distance
}
