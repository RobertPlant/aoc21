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

fn calc_p2(input: &'static str) -> usize {
    let mut invalid = vec![];

    'outer: for line in input.lines() {
        let mut prev = vec![];

        for c in line.chars() {
            let is_closer = vec!['}', ')', ']', '>'].contains(&c);

            if is_closer {
                let p = prev.pop().unwrap();
                match c {
                    ')' => {
                        if p != '(' {
                            continue 'outer;
                        }
                    }
                    ']' => {
                        if p != '[' {
                            continue 'outer;
                        }
                    }
                    '}' => {
                        if p != '{' {
                            continue 'outer;
                        }
                    }
                    '>' => {
                        if p != '<' {
                            continue 'outer;
                        }
                    }
                    _ => {
                        prev.push(p);
                    }
                }
            } else {
                prev.push(c);
            }
        }

        invalid.push(prev);
    }

    let len = invalid.len();
    let mut scores: Vec<usize> = invalid
        .iter()
        .map(|item| {
            item.iter().rev().fold(0, |accum, c| {
                let mut score = accum;

                score = score * 5;

                match c {
                    '(' => {
                        score += 1;
                    }
                    '[' => {
                        score += 2;
                    }
                    '{' => {
                        score += 3;
                    }
                    '<' => {
                        score += 4;
                    }
                    _ => (),
                }

                score
            })
        })
        .collect::<Vec<usize>>();

    scores.sort();

    scores[len / 2]
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

    #[test]
    fn test_p2() {
        assert_eq!(
            calc_p2(
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
            288957
        )
    }
}
