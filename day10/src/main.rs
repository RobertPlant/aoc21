mod input;

fn calc(input: &'static str) -> usize {
    let mut score = 0;

    'outer: for line in input.lines() {
        let mut prev = vec![];

        for c in line.chars() {
            let is_closer = vec!['}', ')', ']', '>'].contains(&c);

            if is_closer {
                let p = prev.pop().unwrap();
                match c {
                    ')' => {
                        if p != '(' {
                            score += 3;
                            continue 'outer;
                        }
                    }
                    ']' => {
                        if p != '[' {
                            score += 57;
                            continue 'outer;
                        }
                    }
                    '}' => {
                        if p != '{' {
                            score += 1197;
                            continue 'outer;
                        }
                    }
                    '>' => {
                        if p != '<' {
                            score += 25137;
                            continue 'outer;
                        }
                    }
                    _ => (),
                }
            } else {
                prev.push(c);
            }
        }
    }

    score
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
        assert_eq!(
            calc(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            ),
            26397
        )
    }
}
