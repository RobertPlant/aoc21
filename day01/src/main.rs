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

fn main() {
    let input_data = input::get_input();

    let total = check_increment(input_data);

    println!("Found {:?}", total);
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
}
