mod input;

fn spawn(input: &'static str, days: usize) -> Vec<usize> {
    let mut parsed: Vec<usize> = input
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    let mut new = 0;

    for _ in 0..days {
        for _ in 0..new {
            parsed.push(9);
        }
        new = 0;

        parsed.iter_mut().for_each(|p| {
            if *p == 0 {
                *p = 6;
            } else {
                if *p == 1 {
                    new += 1;
                }
                *p -= 1;
            }
        });
    }

    parsed
}

#[derive(Clone, Debug, PartialEq)]
struct SpawnTracker {
    count_0: usize,
    count_1: usize,
    count_2: usize,
    count_3: usize,
    count_4: usize,
    count_5: usize,
    count_6: usize,
    count_7: usize,
    count_8: usize,
}

fn spawn_p2(input: &'static str, days: usize) -> SpawnTracker {
    let parsed: Vec<usize> = input
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    let mut spawn_tracker = SpawnTracker {
        count_0: 0,
        count_1: 0,
        count_2: 0,
        count_3: 0,
        count_4: 0,
        count_5: 0,
        count_6: 0,
        count_7: 0,
        count_8: 0,
    };
    parsed.iter().for_each(|p| match p {
        0 => spawn_tracker.count_0 += 1,
        1 => spawn_tracker.count_1 += 1,
        2 => spawn_tracker.count_2 += 1,
        3 => spawn_tracker.count_3 += 1,
        4 => spawn_tracker.count_4 += 1,
        5 => spawn_tracker.count_5 += 1,
        6 => spawn_tracker.count_6 += 1,
        7 => spawn_tracker.count_7 += 1,
        8 => spawn_tracker.count_8 += 1,
        _ => (),
    });

    let mut new = 0;

    for _ in 0..days {
        let clone = spawn_tracker.clone();
        spawn_tracker.count_0 = clone.count_1;
        spawn_tracker.count_1 = clone.count_2;
        spawn_tracker.count_2 = clone.count_3;
        spawn_tracker.count_3 = clone.count_4;
        spawn_tracker.count_4 = clone.count_5;
        spawn_tracker.count_5 = clone.count_6;
        spawn_tracker.count_6 = clone.count_7 + clone.count_0;
        spawn_tracker.count_7 = clone.count_8;

        spawn_tracker.count_8 = new;
        new = clone.count_1;
    }

    spawn_tracker
}

fn calc(input: &'static str, days: usize) -> usize {
    let parsed = spawn(input, days);

    parsed.len()
}

fn calc_p2(input: &'static str, days: usize) -> usize {
    let spawn_tracker = spawn_p2(input, days);

    spawn_tracker.count_0
        + spawn_tracker.count_1
        + spawn_tracker.count_2
        + spawn_tracker.count_3
        + spawn_tracker.count_4
        + spawn_tracker.count_5
        + spawn_tracker.count_6
        + spawn_tracker.count_7
        + spawn_tracker.count_8
}

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data, 80));
    println!("Found P2 {:?}", calc_p2(input_data, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        assert_eq!(spawn("3,4,3,1,2", 1), vec![2, 3, 2, 0, 1])
    }

    #[test]
    fn test_day2() {
        assert_eq!(spawn("3,4,3,1,2", 2), vec![1, 2, 1, 6, 0, 8])
    }

    #[test]
    fn test_day18() {
        assert_eq!(
            spawn("3,4,3,1,2", 18),
            vec![6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8]
        )
    }

    #[test]
    fn test_80_days() {
        assert_eq!(calc("3,4,3,1,2", 80), 5934)
    }

    #[test]
    fn test_spawn2_1_days() {
        assert_eq!(
            spawn_p2("3,4,3,1,2", 1),
            SpawnTracker {
                count_0: 1,
                count_1: 1,
                count_2: 2,
                count_3: 1,
                count_4: 0,
                count_5: 0,
                count_6: 0,
                count_7: 0,
                count_8: 0,
            }
        )
    }

    #[test]
    fn test_spawn2_2_days() {
        assert_eq!(
            spawn_p2("3,4,3,1,2", 2),
            SpawnTracker {
                count_0: 1,
                count_1: 2,
                count_2: 1,
                count_3: 0,
                count_4: 0,
                count_5: 0,
                count_6: 1,
                count_7: 0,
                count_8: 1,
            }
        )
    }

    #[test]
    fn test_spawn2_5_days() {
        assert_eq!(
            spawn_p2("3,4,3,1,2", 5),
            SpawnTracker {
                count_0: 0,
                count_1: 0,
                count_2: 0,
                count_3: 1,
                count_4: 1,
                count_5: 3,
                count_6: 2,
                count_7: 2,
                count_8: 1,
            }
        )
    }

    #[test]
    fn test_spawn2_9_days() {
        assert_eq!(
            spawn_p2("3,4,3,1,2", 9),
            SpawnTracker {
                count_0: 1,
                count_1: 3,
                count_2: 2,
                count_3: 2,
                count_4: 1,
                count_5: 0,
                count_6: 1,
                count_7: 0,
                count_8: 1,
            }
        )
    }

    #[test]
    fn test_spawn2_18_days() {
        assert_eq!(
            spawn_p2("3,4,3,1,2", 18),
            SpawnTracker {
                count_0: 3,
                count_1: 5,
                count_2: 3,
                count_3: 2,
                count_4: 2,
                count_5: 1,
                count_6: 5,
                count_7: 1,
                count_8: 4,
            }
        )
    }

    #[test]
    fn test_spawn2_80_days() {
        assert_eq!(calc_p2("3,4,3,1,2", 80), 5934)
    }

    #[test]
    fn test_spawn2_256_days() {
        assert_eq!(calc_p2("3,4,3,1,2", 256), 26984457539)
    }
}
