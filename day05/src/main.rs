use std::cmp::{max, min};

mod input;

fn calc(input: &'static str, size: usize) -> usize {
    let parsed_vents = input.lines().map(|l| {
        let mut split = l.split(" -> ");
        let start = split.next().unwrap();
        let mut start_split = start.split(",");
        let start_x = start_split.next().unwrap().parse::<usize>().unwrap();
        let start_y = start_split.next().unwrap().parse::<usize>().unwrap();
        let end = split.next().unwrap();
        let mut end_split = end.split(",");
        let end_x = end_split.next().unwrap().parse::<usize>().unwrap();
        let end_y = end_split.next().unwrap().parse::<usize>().unwrap();

        (start_x, start_y, end_x, end_y)
    });
    let filtered_vents = parsed_vents.filter(|l| l.0 == l.2 || l.1 == l.3);
    let mut board = vec![vec![0; size]; size];

    filtered_vents.for_each(|v| {
        if v.1 != v.3 {
            for y in min(v.1, v.3)..=max(v.1, v.3) {
                board[y][v.0] += 1;
            }
        }

        if v.0 != v.2 {
            for x in min(v.0, v.2)..=max(v.0, v.2) {
                board[v.1][x] += 1;
            }
        }
    });

    get_board_score(&board)
}

fn get_board_score(board: &Vec<Vec<usize>>) -> usize {
    let mut score = 0;

    board
        .iter()
        .for_each(|l| score += l.iter().filter(|&&s| s > 1).count());

    score
}

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data, 1000));
    // println!("Found P2 {:?}", calc_p2(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            calc(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
",
                10
            ),
            5
        )
    }
}
