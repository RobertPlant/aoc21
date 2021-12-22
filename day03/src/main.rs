mod input;

fn calc(input: &'static str, length: usize) -> usize {
    let middle = input.lines().count() / 2;
    let mut accumulator: Vec<u32> = Vec::with_capacity(length);
    let mut epsilon_string = String::from("");
    let mut gamma_string = String::from("");

    for _ in 0..length {
        accumulator.push(0)
    }

    for line in input.lines() {
        let mut key = 0;
        for char in line.chars() {
            let parsed = char.to_digit(10).unwrap();
            accumulator[key] = accumulator[key] + parsed;
            key = key + 1;
        }
    }

    for acc in accumulator {
        let value = acc.try_into().unwrap();
        epsilon_string.push(if middle < value { '1' } else { '0' });
        gamma_string.push(if middle > value { '1' } else { '0' });
    }

    let epsilon = isize::from_str_radix(&epsilon_string, 2).unwrap();
    let gamma = isize::from_str_radix(&gamma_string, 2).unwrap();

    (gamma * epsilon).try_into().unwrap()
}

fn main() {
    let input_data = input::get_input();

    let total = calc(input_data, 12);

    println!("Found {:?}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            calc(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
                5,
            ),
            198
        )
    }
}
