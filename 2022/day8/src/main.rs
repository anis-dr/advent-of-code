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
    let mut left_distance = 0;
    for c in (0..col).rev() {
        left_distance += 1;
        if grid[row][c] >= tree {
            break;
        }
    }

    // right
    let mut right_distance = 0;
    for c in col + 1..cols {
        right_distance += 1;
        if grid[row][c] >= tree {
            break;
        }
    }

    // up
    let mut up_distance = 0;
    for r in (0..row).rev() {
        up_distance += 1;
        if grid[r][col] >= tree {
            break;
        }
    }

    // down
    let mut down_distance = 0;
    for r in row + 1..rows {
        down_distance += 1;
        if grid[r][col] >= tree {
            break;
        }
    }

    // return the multiplication of the distances
    left_distance * right_distance * up_distance * down_distance
}