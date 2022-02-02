mod input;

fn calc(input: &'static str, days: usize) -> usize {
    let mut grid: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let mut flash_count = 0;

    for _ in 0..days {
        grid = grid
            .iter()
            .map(|line| line.iter().map(|item| item + 1).collect())
            .collect();

        grid = flash(grid);

        flash_count += grid
            .iter()
            .map(|l| {
                l.iter()
                    .map(|i| if *i >= 200 { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>();

        grid = grid
            .iter()
            .map(|line| {
                line.iter()
                    .map(|item| if *item > 9 { 0 } else { *item })
                    .collect()
            })
            .collect();
    }

    flash_count
}

fn flash(grid: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let updated_grid: Vec<Vec<usize>> = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|item| if *item > 9 && *item < 100 { 100 } else { *item })
                .collect()
        })
        .collect();
    let height = grid.len() - 1;
    let width = grid[0].len() - 1;

    let mut flashed_grid = updated_grid.clone();

    for (row_i, row) in updated_grid.iter().enumerate() {
        for (column_i, column) in row.iter().enumerate() {
            if *column >= 100 && *column < 200 {
                if row_i != 0 && column_i != 0 {
                    flashed_grid[row_i - 1][column_i - 1] += 1;
                }
                if row_i != 0 {
                    flashed_grid[row_i - 1][column_i] += 1;
                }
                if row_i != 0 && column_i < width {
                    flashed_grid[row_i - 1][column_i + 1] += 1;
                }
                if column_i < width {
                    flashed_grid[row_i][column_i + 1] += 1;
                }
                if row_i < height && column_i < width {
                    flashed_grid[row_i + 1][column_i + 1] += 1;
                }
                if row_i < height {
                    flashed_grid[row_i + 1][column_i] += 1;
                }
                if row_i < height && column_i != 0 {
                    flashed_grid[row_i + 1][column_i - 1] += 1;
                }
                if column_i != 0 {
                    flashed_grid[row_i][column_i - 1] += 1;
                }

                flashed_grid[row_i][column_i] = 200;
            }
        }
    }

    let mut will_flash_again = false;
    grid.iter().for_each(|line| {
        line.iter().for_each(|item| {
            if *item > 9 && *item < 100 {
                will_flash_again = true
            }
        })
    });

    if will_flash_again {
        return flash(flashed_grid);
    }

    flashed_grid
}

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data, 100));
    // println!("Found P2 {:?}", calc_p2(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
    }

    #[test]
    fn test_1() {
        assert_eq!(calc(get_input(), 1), 0)
    }

    #[test]
    fn test_2() {
        assert_eq!(calc(get_input(), 2), 35)
    }

    #[test]
    fn test_3() {
        assert_eq!(calc(get_input(), 3), 80)
    }

    #[test]
    fn test_4() {
        assert_eq!(calc(get_input(), 4), 96)
    }
}
