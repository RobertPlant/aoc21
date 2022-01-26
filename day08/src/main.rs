mod input;
fn parse(input: &'static str, position: usize) -> Vec<&'static str> {
    input
        .lines()
        .map(|l| l.split(" | ").nth(position).unwrap())
        .collect::<Vec<&'static str>>()
        .iter()
        .map(|i| i.split(" "))
        .flatten()
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

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data));
    // println!("Found P2 {:?}", calc_p2(input_data));
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
}
