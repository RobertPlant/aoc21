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

fn find_oxygen(input: Vec<&str>, max: u32, pos: usize, default_search: u32) -> Vec<&str> {
    let mut val = 0;
    let mut search = default_search;
    let mut lines: Vec<&str> = Vec::with_capacity((max).try_into().unwrap());

    for line in input.clone() {
        let mut iter = line.chars();
        let char = iter.nth(pos).unwrap();
        let parsed = char.to_digit(10).unwrap();

        val = val + parsed;
    }

    if val >= (max - val) {
        search = if default_search == 1 { 0 } else { 1 };
    }

    let mut key = 0;
    for line in input {
        let mut iter = line.chars();
        let char = iter.nth(pos).unwrap();
        let parsed = char.to_digit(10).unwrap();

        match parsed {
            val if val == search => {
                lines.push(line.chars().as_str());
            }
            _ => (),
        };

        key = key + 1;
    }

    if lines.len() != 1 {
        return find_oxygen(
            lines.clone(),
            (lines.len()).try_into().unwrap(),
            pos + 1,
            default_search,
        );
    } else {
        return lines;
    }
}

fn calc_p2(input: &'static str) -> usize {
    let max: u32 = (input.lines().count()).try_into().unwrap();

    let mut oxygen_vec = find_oxygen(input.lines().collect(), max, 0, 0);
    let oxygen = isize::from_str_radix(oxygen_vec.pop().unwrap(), 2).unwrap();

    let mut carbon_dioxide_vec = find_oxygen(input.lines().collect(), max, 0, 1);
    let carbon_dioxide = isize::from_str_radix(carbon_dioxide_vec.pop().unwrap(), 2).unwrap();

    (oxygen * carbon_dioxide).try_into().unwrap()
}

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data, 12));
    println!("Found P2 {:?}", calc_p2(input_data));
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

    #[test]
    fn test_p2() {
        assert_eq!(
            calc_p2(
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
            ),
            230
        )
    }
}
