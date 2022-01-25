use std::cmp::{max, min};

mod input;

fn calc(input: &'static str) -> usize {
    let mut parsed: Vec<usize> = input
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    parsed.sort();
    let median = parsed.iter().nth(parsed.len() / 2).unwrap();

    parsed.iter().map(|v| max(median, v) - min(median, v)).sum()
}

fn calc_p2(input: &'static str) -> usize {
    let parsed: Vec<usize> = input
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    let sum = (parsed.clone().iter().sum::<usize>()) as f32;
    let len = (parsed.len()) as f32;
    let mean = ((sum / len).round()) as usize;

    min(
        get_fuel_cost(&parsed, mean - 1),
        min(
            get_fuel_cost(&parsed, mean),
            get_fuel_cost(&parsed, mean + 1),
        ),
    )
}

fn get_fuel_cost(movement: &Vec<usize>, mean: usize) -> usize {
    movement
        .iter()
        .map(|v| {
            let distance = max(mean, *v) - min(mean, *v);

            (distance * (distance + 1)) / 2
        })
        .sum::<usize>()
}

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data));
    println!("Found P2 {:?}", calc_p2(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(calc("16,1,2,0,4,2,7,1,2,14"), 37)
    }

    #[test]
    fn test_p2() {
        assert_eq!(calc_p2("16,1,2,0,4,2,7,1,2,14"), 168)
    }
}
