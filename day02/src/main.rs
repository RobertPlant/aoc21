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

fn main() {
    let input_data = input::get_input();

    let total = traverse(input_data);

    println!("Found {:?}", total);
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
}
