mod input;

fn traverse(input: &'static str) -> usize {
    let mut distance = 0;
    let mut depth = 0;

    for line in input.split("\n") {
        let mut key_value = line.split(" ");
        let operator = key_value.next().unwrap();
        let value = key_value.next().unwrap().parse::<i32>().unwrap_or(0);

        match operator {
            "forward" => {
                distance = distance + value;
            }
            "up" => {
                depth = depth - value;
            }
            "down" => {
                depth = depth + value;
            }
            _ => (),
        };
    }

    (distance * depth).try_into().unwrap()
}

fn traverse_p2(input: &'static str) -> usize {
    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.split("\n") {
        let mut key_value = line.split(" ");
        let operator = key_value.next().unwrap();
        let value = key_value.next().unwrap().parse::<i32>().unwrap_or(0);

        match operator {
            "forward" => {
                distance = distance + value;
                depth = (aim * value) + depth;
            }
            "up" => {
                aim = aim - value;
            }
            "down" => {
                aim = aim + value;
            }
            _ => (),
        };
    }

    (distance * depth).try_into().unwrap()
}

fn main() {
    let input_data = input::get_input();

    let total = traverse(input_data);

    println!("Found {:?}", total);

    let total = traverse_p2(input_data);

    println!("Found P2 {:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            traverse(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            ),
            150
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            traverse_p2(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            ),
            900
        )
    }
}
