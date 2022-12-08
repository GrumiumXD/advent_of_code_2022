#[derive(Debug)]
struct TreeGrid {
    trees: Vec<u8>,
    width: usize,
    height: usize,
}

fn parse_tree_grid(input: &str) -> TreeGrid {
    let height = input.lines().count();

    let trees = input
        .lines()
        .flat_map(|l| l.chars().map(|c| (c as u8) - 48).collect::<Vec<u8>>())
        .collect::<Vec<u8>>();

    let width = trees.len() / height;

    TreeGrid {
        trees,
        width,
        height,
    }
}

pub fn puzzle_1(input: &str) -> String {
    let grid = parse_tree_grid(input);

    let visibility = grid
        .trees
        .iter()
        .enumerate()
        .filter(|&(idx, tree)| {
            let x = idx % grid.width;
            let y = idx / grid.width;

            // outer ring
            if x == 0 || x == (grid.width - 1) || y == 0 || y == (grid.height - 1) {
                return true;
            }

            let mut west = true;
            for i in 0..x {
                if grid.trees[y * grid.width + i] >= *tree {
                    west = false;
                    break;
                }
            }

            let mut east = true;
            for i in (x + 1)..grid.width {
                if grid.trees[y * grid.width + i] >= *tree {
                    east = false;
                    break;
                }
            }

            let mut north = true;
            for i in 0..y {
                if grid.trees[i * grid.width + x] >= *tree {
                    north = false;
                    break;
                }
            }

            let mut south = true;
            for i in (y + 1)..grid.height {
                if grid.trees[i * grid.width + x] >= *tree {
                    south = false;
                    break;
                }
            }

            // println!(
            //     "tree: {} N:{} E:{} S:{} W:{}",
            //     idx, north, east, south, west
            // );

            north || east || south || west
        })
        .count();

    visibility.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let grid = parse_tree_grid(input);

    let scores = grid
        .trees
        .iter()
        .enumerate()
        .map(|(idx, tree)| {
            let x = idx % grid.width;
            let y = idx / grid.width;

            // outer ring
            if x == 0 || x == (grid.width - 1) || y == 0 || y == (grid.height - 1) {
                return 0;
            }

            let mut west = 0;
            for i in (0..x).rev() {
                west += 1;
                if grid.trees[y * grid.width + i] >= *tree {
                    break;
                }
            }

            let mut east = 0;
            for i in (x + 1)..grid.width {
                east += 1;
                if grid.trees[y * grid.width + i] >= *tree {
                    break;
                }
            }

            let mut north = 0;
            for i in (0..y).rev() {
                north += 1;
                if grid.trees[i * grid.width + x] >= *tree {
                    break;
                }
            }

            let mut south = 0;
            for i in (y + 1)..grid.height {
                south += 1;
                if grid.trees[i * grid.width + x] >= *tree {
                    break;
                }
            }

            // println!(
            //     "tree: {} N:{} E:{} S:{} W:{}",
            //     idx, north, east, south, west
            // );

            north * east * south * west
        })
        .collect::<Vec<u32>>();

    let max = scores.iter().max().unwrap();

    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "8");
    }
}
