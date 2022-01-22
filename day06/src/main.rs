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

fn calc(input: &'static str, days: usize) -> usize {
    let parsed = spawn(input, days);

    parsed.len()
}

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data, 80));
    // println!("Found P2 {:?}", calc_p2(input_data, 1000));
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
}
