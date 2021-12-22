mod input;

fn check_increment(input: Vec<isize>) -> isize {
    let mut last = 99999;
    let mut increase_count = 0;

    for depth in input {
        if depth > last {
            increase_count = increase_count + 1;
        }

        last = depth;
    }

    increase_count
}

fn check_increment_p2(input: Vec<isize>) -> isize {
    let mut last3 = 99999;
    let mut last2 = 99999;
    let mut last1 = 99999;
    let mut increase_count = 0;

    for depth in input {
        let current = depth + last2 + last1;
        let previous = last3 + last2 + last1;

        if current > previous {
            increase_count = increase_count + 1;
        }

        last3 = last2;
        last2 = last1;
        last1 = depth;
    }

    increase_count
}

fn main() {
    let input_data = input::get_input();

    let total = check_increment(input_data.clone());

    println!("Found {:?}", total);

    let total_p2 = check_increment_p2(input_data.clone());

    println!("Found P2 {:?}", total_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            check_increment(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            check_increment_p2(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        )
    }
}
