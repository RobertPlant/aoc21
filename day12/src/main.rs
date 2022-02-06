mod input;

fn routes(input: Vec<Node>) -> Vec<String> {
    let mut routes = vec![];
    let start_id = input.iter().position(|n| n.id == "start").unwrap();

    for link in input[start_id].links.iter() {
        let link = input.iter().nth(*link).unwrap();

        delve(
            &input,
            link,
            &mut routes,
            input[start_id].id.clone() + "," + &link.id + ",",
        );
    }

    routes.sort();

    routes
}

fn delve(input: &Vec<Node>, target: &Node, routes: &mut Vec<String>, chain: String) {
    for id in target.links.iter() {
        let target = input.iter().nth(*id).unwrap();
        let current_chain = chain.clone() + &target.id + ",";
        let search = ",".to_owned() + &target.id + &",".to_owned();

        if !target.big_cave && chain.contains(&search) {
            continue;
        }

        if target.id == "end" {
            routes.push(chain.clone() + &target.id);
        }

        delve(input, target, routes, current_chain);
    }
}

fn parse(input: &'static str) -> Vec<Node> {
    let mut nodes = vec![];
    let mut inserted = vec![];

    input.lines().for_each(|line| {
        line.split("-").for_each(|s| {
            let id = s.to_string();
            let big_cave = id.to_uppercase() == id;

            if !inserted.contains(&id) {
                inserted.push(id.clone());
                nodes.push(Node {
                    id,
                    big_cave,
                    links: vec![],
                });
            }
        });
    });

    input.lines().for_each(|line| {
        let mut split = line.split("-");
        let start_key = split.next().unwrap().to_string();
        let end_key = split.next().unwrap().to_string();

        let start_id = nodes.iter().position(|n| n.id == start_key).unwrap();
        let end_id = nodes.iter().position(|n| n.id == end_key).unwrap();

        if end_key != "end" && start_key != "start" {
            nodes[end_id].links.push(start_id);
        }
        if start_key != "end" && end_key != "start" {
            nodes[start_id].links.push(end_id);
        }
    });

    nodes
}
fn routes_p2(input: Vec<Node>) -> Vec<String> {
    let mut routes = vec![];
    let start_id = input.iter().position(|n| n.id == "start").unwrap();

    for link in input[start_id].links.iter() {
        let link = input.iter().nth(*link).unwrap();

        delve_p2(
            &input,
            link,
            &mut routes,
            input[start_id].id.clone() + "," + &link.id + ",",
        );
    }

    routes.sort();

    routes
}

fn delve_p2(input: &Vec<Node>, target: &Node, routes: &mut Vec<String>, chain: String) {
    for id in target.links.iter() {
        let target = input.iter().nth(*id).unwrap();
        let current_chain = chain.clone() + &target.id + ",";
        let search = ",".to_owned() + &target.id + &",".to_owned();
        let mut visited_small_already = false;
        input
            .iter()
            .filter(|n| !n.big_cave && n.id != "end" && n.id != "start")
            .for_each(|n| {
                if !visited_small_already {
                    let node_search_term = ",".to_owned() + &n.id + &",".to_owned();
                    visited_small_already = chain.matches(&node_search_term).count() > 1;
                }
            });

        if !target.big_cave && chain.contains(&search) && visited_small_already {
            continue;
        }

        if target.id == "end" {
            routes.push(chain.clone() + &target.id);
        }

        delve_p2(input, target, routes, current_chain);
    }
}

fn calc(input: &'static str) -> usize {
    routes(parse(input)).len()
}

fn calc_p2(input: &'static str) -> usize {
    routes_p2(parse(input)).len()
}

fn main() {
    let input_data = input::get_input();

    println!("Found {:?}", calc(input_data));
    println!("Found P2 {:?}", calc_p2(input_data));
}

#[derive(Debug, PartialEq)]
struct Node {
    id: String,
    big_cave: bool,
    links: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(
                "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            ),
            vec![
                Node {
                    id: "start".to_string(),
                    big_cave: false,
                    links: vec![1, 2],
                },
                Node {
                    id: "A".to_string(),
                    big_cave: true,
                    links: vec![3, 2, 5],
                },
                Node {
                    id: "b".to_string(),
                    big_cave: false,
                    links: vec![1, 4, 5],
                },
                Node {
                    id: "c".to_string(),
                    big_cave: false,
                    links: vec![1],
                },
                Node {
                    id: "d".to_string(),
                    big_cave: false,
                    links: vec![2],
                },
                Node {
                    id: "end".to_string(),
                    big_cave: false,
                    links: vec![],
                },
            ]
        )
    }

    #[test]
    fn test_routes() {
        assert_eq!(
            routes(vec![
                Node {
                    id: "start".to_string(),
                    big_cave: false,
                    links: vec![1, 3],
                },
                Node {
                    id: "A".to_string(),
                    big_cave: true,
                    links: vec![2, 3, 5],
                },
                Node {
                    id: "c".to_string(),
                    big_cave: false,
                    links: vec![1],
                },
                Node {
                    id: "b".to_string(),
                    big_cave: false,
                    links: vec![1, 4, 5],
                },
                Node {
                    id: "d".to_string(),
                    big_cave: false,
                    links: vec![3],
                },
                Node {
                    id: "end".to_string(),
                    big_cave: false,
                    links: vec![],
                },
            ]),
            vec![
                "start,A,b,A,c,A,end",
                "start,A,b,A,end",
                "start,A,b,end",
                "start,A,c,A,b,A,end",
                "start,A,c,A,b,end",
                "start,A,c,A,end",
                "start,A,end",
                "start,b,A,c,A,end",
                "start,b,A,end",
                "start,b,end",
            ]
        )
    }

    #[test]
    fn test_calc() {
        assert_eq!(
            calc(
                "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            ),
            10
        )
    }

    #[test]
    fn test_calc_2() {
        assert_eq!(
            calc(
                "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            ),
            19
        )
    }

    #[test]
    fn test_calc_p2() {
        assert_eq!(
            calc_p2(
                "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            ),
            36
        )
    }

    #[test]
    fn test_calc_p2_2() {
        assert_eq!(
            calc_p2(
                "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            ),
            103
        )
    }
}
