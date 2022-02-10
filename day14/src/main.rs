use std::collections::HashMap;

mod input;

fn calc(input: &'static str, num_of_iterations: usize) -> HashMap<String, usize> {
    let mut split_input = input.split_terminator("\n\n");

    let template = split_input.next().unwrap();
    let mutations = split_input
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .collect::<Vec<(&str, &str)>>();
    let mut found_instances: HashMap<String, usize> =
        mutations.iter().map(|m| (m.0.to_string(), 0)).collect();
    template
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|window| window[0].to_string() + &window[1].to_string())
        .for_each(|pair| *found_instances.entry(pair).or_default() += 1);

    let mut occurances: HashMap<String, usize> = HashMap::new();
    template.chars().for_each(|t| {
        *occurances.entry(t.to_string()).or_default() += 1;
    });

    for _ in 0..num_of_iterations {
        let working_copy = found_instances.clone();

        mutations.iter().for_each(|m| {
            let number_of_occurances = working_copy[m.0];

            if number_of_occurances > 0 {
                *occurances.entry(m.1.to_string()).or_default() += number_of_occurances;

                *found_instances
                    .entry(m.0.chars().nth(0).unwrap().to_string() + m.1)
                    .or_default() += number_of_occurances;

                *found_instances.entry(m.0.to_string()).or_default() -= number_of_occurances;

                *found_instances
                    .entry(m.1.to_string() + &m.0.chars().nth(1).unwrap().to_string())
                    .or_default() += number_of_occurances;
            }
        })
    }

    occurances
}

fn score(occurances: HashMap<String, usize>) -> usize {
    occurances.iter().map(|v| v.1).max().unwrap() - occurances.iter().map(|v| v.1).min().unwrap()
}

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", score(calc(input_data, 10)));
    println!("Found P2 {:?}", score(calc(input_data, 40)));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
    }

    #[test]
    fn test() {
        assert_eq!(
            calc(input(), 1),
            HashMap::from([
                (String::from("H"), 1),
                (String::from("N"), 2),
                (String::from("C"), 2),
                (String::from("B"), 2),
            ]),
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            calc(input(), 2),
            HashMap::from([
                (String::from("H"), 1),
                (String::from("N"), 2),
                (String::from("C"), 4),
                (String::from("B"), 6),
            ]),
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            calc(input(), 3),
            HashMap::from([
                (String::from("H"), 4),
                (String::from("N"), 5),
                (String::from("C"), 5),
                (String::from("B"), 11),
            ]),
        )
    }

    #[test]
    fn test_10() {
        assert_eq!(score(calc(input(), 10)), 1588)
    }

    #[test]
    fn test_30() {
        assert_eq!(score(calc(input(), 40)), 2188189693529)
    }
}
