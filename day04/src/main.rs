mod input;

fn calc(input: &'static str) -> usize {
    let board_offset = 1;
    let board_size = 5;
    let call_sequence: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let board_count = input.lines().filter(|l| l.len() == 0).count();

    let mut solutions: Vec<Vec<usize>> = Vec::with_capacity(board_count);

    let mut line_index = 1;
    for _ in 0..board_count {
        for _ in 0..board_size {
            solutions.push(
                input
                    .lines()
                    .nth(board_offset + line_index)
                    .unwrap()
                    .split_whitespace()
                    .map(|l| l.parse::<usize>().unwrap())
                    .collect(),
            );

            line_index += 1;
        }

        let boards_solution_length = solutions.len();
        for index in 0..board_size {
            solutions.push(vec![
                solutions[boards_solution_length - 5][index],
                solutions[boards_solution_length - 4][index],
                solutions[boards_solution_length - 3][index],
                solutions[boards_solution_length - 2][index],
                solutions[boards_solution_length - 1][index],
            ]);
        }

        line_index += 1;
    }

    for number in call_sequence {
        for (i, r) in solutions.clone().iter().enumerate() {
            for (vi, v) in r.iter().enumerate() {
                if v == &number {
                    solutions[i][vi] = 0;
                }
            }
        }

        for (i, r) in solutions.iter().enumerate() {
            if r.iter().sum::<usize>() == 0 {
                let solution_start_of_points = i / 10;
                let mut sum = 0;

                for r in solutions.iter().skip(solution_start_of_points * 10).take(5) {
                    sum += r.iter().sum::<usize>();
                }

                return sum * number;
            }
        }
    }

    0
}

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data));
    //println!("Found P2 {:?}", calc_p2(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            calc(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
            ),
            4512
        )
    }
}
