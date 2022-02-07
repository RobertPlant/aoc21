use std::cmp::max;

mod input;

fn calc(input: &'static str, num_of_folds: usize) -> usize {
    let mut split_input = input.split_terminator("\n\n");

    let dots = split_input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut split = l.split(",");
            let x = split.next().unwrap().parse::<usize>().unwrap();
            let y = split.next().unwrap().parse::<usize>().unwrap();

            (x, y)
        })
        .collect::<Vec<(usize, usize)>>();
    let folds = split_input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut split = l.split("=");
            if split.next().unwrap() == "fold along y" {
                return (0, split.next().unwrap().parse::<usize>().unwrap());
            }

            (split.next().unwrap().parse::<usize>().unwrap(), 0)
        })
        .collect::<Vec<(usize, usize)>>();

    let width = dots.iter().fold(0, |acc, v| max(acc, v.0)) + 1;
    let height = dots.iter().fold(0, |acc, v| max(acc, v.1)) + 1;

    let mut grid = vec![vec![0; width]; height];

    dots.iter().for_each(|d| grid[d.1][d.0] = 1);

    folds.iter().take(num_of_folds).for_each(|f| {
        if f.0 > 0 {
            let second_half = grid
                .iter_mut()
                .map(|l| l.split_off(f.0 + 1))
                .collect::<Vec<Vec<usize>>>();

            grid.iter_mut().for_each(|l| l.resize(f.0, 0));

            for (row_i, row) in second_half.iter().enumerate() {
                for (column_i, column) in row.iter().rev().enumerate() {
                    grid[row_i][column_i] = max(grid[row_i][column_i], *column);
                }
            }
        }

        if f.1 > 0 {
            let second_half = grid.split_off(f.1 + 1);
            grid.resize(f.1, vec![0]);

            for (row_i, row) in second_half.iter().rev().enumerate() {
                for (column_i, column) in row.iter().enumerate() {
                    grid[row_i][column_i] = max(grid[row_i][column_i], *column);
                }
            }
        }
    });

    grid.iter().map(|r| r.iter().sum::<usize>()).sum()
}

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data, 1));
    // println!("Found P2 {:?}", calc_p2(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            calc(
                "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5",
                1
            ),
            17
        )
    }
    #[test]
    fn test_x() {
        assert_eq!(
            calc(
                "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5",
                2
            ),
            16
        )
    }
}
