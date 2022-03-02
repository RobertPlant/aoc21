use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

mod input;

fn calc(input: &'static str, goal: (usize, usize)) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let result = dijkstra(
        &(0, 0),
        |&(x, y)| {
            let mut vec = vec![(x + 1, y), (x, y + 1)];

            if x > 2 {
                vec.append(&mut vec![(x - 1, y)]);
            };

            if y > 2 {
                vec.append(&mut vec![(x, y - 1)]);
            };

            vec.into_iter().map(|p| {
                if p.0 == (goal.0 + 1) || p.1 == (goal.1 + 1) {
                    return (p, 999);
                }

                (p, grid[p.0][p.1])
            })
        },
        |&p| p == goal,
    );

    result.unwrap().1
}

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data, (99, 99)));
    //println!("Found P2 {:?}", score(calc(input_data, 40)));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
    }

    #[test]
    fn test() {
        assert_eq!(calc(input(), (9, 9)), 40)
    }
}
