use itertools::Itertools;

mod input;

fn parse(input: &'static str, position: usize) -> Vec<String> {
    input
        .lines()
        .map(|l| l.split(" | ").nth(position).unwrap())
        .collect::<Vec<&'static str>>()
        .iter()
        .map(|i| i.split(" "))
        .flatten()
        .map(|v| v.chars().sorted().collect())
        .collect()
}

fn calc(input: &'static str) -> usize {
    parse(input, 1)
        .iter()
        .map(|o| match o.len() {
            2 => 1,
            4 => 1,
            3 => 1,
            7 => 1,
            _ => 0,
        })
        .sum()
}

fn calc_p2(input: &'static str) -> usize {
    input.lines().map(|line| row(line)).sum()
}

fn row(input: &'static str) -> usize {
    let mut one = String::from("");
    let mut seven = String::from("");
    let mut four = String::from("");
    let mut eight = String::from("");
    let mut two_three_or_five: Vec<String> = vec![];
    let mut zero_six_or_nine: Vec<String> = vec![];

    parse(input, 0).iter().for_each(|v| {
        let o = v.to_string();

        match v.len() {
            2 => {
                if one == "" {
                    one = o;
                }
            }
            3 => {
                if seven == "" {
                    seven = o;
                }
            }
            4 => {
                if four == "" {
                    four = o;
                }
            }
            5 => {
                if !two_three_or_five.contains(&o) {
                    two_three_or_five.push(o);
                }
            }
            6 => {
                if !zero_six_or_nine.contains(&o) {
                    zero_six_or_nine.push(o);
                }
            }
            7 => {
                if eight == "" {
                    eight = o;
                }
            }
            _ => (),
        }
    });

    let three = find(&two_three_or_five, &one, 2);
    let five_or_two: Vec<String> = two_three_or_five
        .into_iter()
        .filter(|v| **v != three)
        .collect();
    let five = find(&five_or_two, &four, 3);
    let two: String = five_or_two
        .iter()
        .find(|v| **v != five)
        .unwrap()
        .to_string();
    let nine = find(&zero_six_or_nine, &three, 5);
    let six = &multi_level_find(&zero_six_or_nine, &five, 5)
        .iter()
        .find(|v| **v != nine)
        .unwrap()
        .clone();
    let zero: String = zero_six_or_nine
        .iter()
        .find(|v| **v != *six && **v != nine)
        .unwrap()
        .to_string();

    let data: String = parse(input, 1)
        .iter()
        .map(|val| {
            if zero == *val {
                return "0";
            }
            if one == *val {
                return "1";
            }
            if two == *val {
                return "2";
            }
            if three == *val {
                return "3";
            }
            if four == *val {
                return "4";
            }
            if five == *val {
                return "5";
            }
            if six == val {
                return "6";
            }
            if seven == *val {
                return "7";
            }
            if eight == *val {
                return "8";
            }
            if nine == *val {
                return "9";
            }

            panic!()
        })
        .flat_map(|s| s.chars())
        .collect();

    data.parse::<usize>().unwrap()
}

fn find(vec: &Vec<String>, string: &String, search: usize) -> String {
    for val in vec {
        let mut found_count = 0;

        for v in val.chars() {
            for char in string.chars() {
                if char == v {
                    found_count += 1;
                }
            }
        }

        if found_count == search {
            return val.to_string();
        }
    }

    panic!()
}

fn multi_level_find(vec: &Vec<String>, string: &String, search: usize) -> Vec<String> {
    let mut results: Vec<String> = vec![];

    for val in vec {
        let mut found_count = 0;

        for v in val.chars() {
            for char in string.chars() {
                if char == v {
                    found_count += 1;
                }
            }
        }

        if found_count == search {
            results.push(val.to_string());
        }
    }

    results
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
        assert_eq!(calc("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce "), 26)
    }

    #[test]
    fn test_p2_1() {
        assert_eq!(row("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"), 8394)
    }

    #[test]
    fn test_p2_2() {
        assert_eq!(row("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"), 9781)
    }

    #[test]
    fn test_p2_3() {
        assert_eq!(
            row("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
            1197
        )
    }

    #[test]
    fn test_p2_4() {
        assert_eq!(row("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"), 9361)
    }

    #[test]
    fn test_p2_5() {
        assert_eq!(row("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"), 4873)
    }

    #[test]
    fn test_p2_6() {
        assert_eq!(row("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"), 8418)
    }

    #[test]
    fn test_p2_7() {
        assert_eq!(row("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"), 4548)
    }

    #[test]
    fn test_p2_8() {
        assert_eq!(row("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"), 1625)
    }

    #[test]
    fn test_p2_9() {
        assert_eq!(
            row("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
            8717
        )
    }

    #[test]
    fn test_p2_10() {
        assert_eq!(
            row("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"),
            4315
        )
    }
}
