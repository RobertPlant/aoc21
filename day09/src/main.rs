mod input;

fn calc_p2(input: &'static str) -> usize {
    let values: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    let height = values.len() - 1;
    let width = values[0].len() - 1;
    let mut basins = vec![];

    for (row_i, row) in values.iter().enumerate() {
        for (column_i, column) in row.iter().enumerate() {
            let mut found = true;

            if row_i != 0 && values[row_i - 1][column_i] <= *column {
                found = false;
            }

            if row_i != height && values[row_i + 1][column_i] <= *column {
                found = false;
            }

            if column_i != 0 && values[row_i][column_i - 1] <= *column {
                found = false;
            }

            if column_i != width && values[row_i][column_i + 1] <= *column {
                found = false;
            }

            if found {
                basins.push(find_basin_size(&values, row_i, column_i, &mut vec![]) - 1);
            }
        }
    }

    basins.sort();
    let len = basins.len();

    basins[(len - 3)..len].to_vec().iter().product()
}

fn find_basin_size(
    values: &Vec<Vec<usize>>,
    row_i: usize,
    column_i: usize,
    found: &mut Vec<(usize, usize)>,
) -> usize {
    let mut size = 1;
    let has_upper = row_i != 0;
    let has_lower = row_i != values.len() - 1;
    let has_left = column_i != 0;
    let has_right = column_i != values[0].len() - 1;
    let value = *values
        .iter()
        .nth(row_i)
        .unwrap()
        .iter()
        .nth(column_i)
        .unwrap();
    let new_value = value + 1;
    if new_value == 9 {
        return size;
    }

    size += if has_upper && !found.contains(&(row_i - 1, column_i)) {
        if *values
            .iter()
            .nth(row_i - 1)
            .unwrap()
            .iter()
            .nth(column_i)
            .unwrap()
            != 9
        {
            found.push((row_i - 1, column_i));
            find_basin_size(&values, row_i - 1, column_i, found)
        } else {
            0
        }
    } else {
        0
    };
    size += if has_lower && !found.contains(&(row_i + 1, column_i)) {
        if *values
            .iter()
            .nth(row_i + 1)
            .unwrap()
            .iter()
            .nth(column_i)
            .unwrap()
            != 9
        {
            found.push((row_i + 1, column_i));
            find_basin_size(&values, row_i + 1, column_i, found)
        } else {
            0
        }
    } else {
        0
    };
    size += if has_left && !found.contains(&(row_i, column_i - 1)) {
        if *values
            .iter()
            .nth(row_i)
            .unwrap()
            .iter()
            .nth(column_i - 1)
            .unwrap()
            != 9
        {
            found.push((row_i, column_i - 1));
            find_basin_size(&values, row_i, column_i - 1, found)
        } else {
            0
        }
    } else {
        0
    };
    size += if has_right && !found.contains(&(row_i, column_i + 1)) {
        if *values
            .iter()
            .nth(row_i)
            .unwrap()
            .iter()
            .nth(column_i + 1)
            .unwrap()
            != 9
        {
            found.push((row_i, column_i + 1));
            find_basin_size(&values, row_i, column_i + 1, found)
        } else {
            0
        }
    } else {
        0
    };

    size
}

fn calc(input: &'static str) -> usize {
    let values: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    let height = values.len() - 1;
    let width = values[0].len() - 1;
    let mut sum = 0;

    for (row_i, row) in values.iter().enumerate() {
        for (column_i, column) in row.iter().enumerate() {
            let mut found = true;

            if row_i != 0 && values[row_i - 1][column_i] <= *column {
                found = false;
            }

            if row_i != height && values[row_i + 1][column_i] <= *column {
                found = false;
            }

            if column_i != 0 && values[row_i][column_i - 1] <= *column {
                found = false;
            }

            if column_i != width && values[row_i][column_i + 1] <= *column {
                found = false;
            }

            if found {
                sum += column + 1;
            }
        }
    }

    sum
}

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data));
    println!("Found P2 {:?}", calc_p2(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            calc(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            ),
            15
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            calc(
                "2199943210
3987894921
9856789892
8767896781
9899965670"
            ),
            16
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            calc(
                "2199943210
3987894921
9856783892
8767896781
9899965670"
            ),
            20
        )
    }

    #[test]
    fn test_4() {
        assert_eq!(
            calc(
                "0199943210
3987894921
9856783892
8767896781
5899965670"
            ),
            25
        )
    }

    #[test]
    fn test_5() {
        assert_eq!(
            calc(
                "0099943210
3987894921
9856783892
8767896781
5899465670"
            ),
            29
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            calc_p2(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            ),
            1134
        )
    }
}
