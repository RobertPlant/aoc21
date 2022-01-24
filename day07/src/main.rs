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

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data));
    //println!("Found P2 {:?}", calc_p2(input_data, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(calc("16,1,2,0,4,2,7,1,2,14"), 37)
    }
}
