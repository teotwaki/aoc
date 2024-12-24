use common::Answer;
use rustc_hash::{FxHashMap, FxHashSet};

fn parse(s: &str) -> Network {
    let mut nodes: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();

    s.lines()
        .map(|l| {
            let mut parts = l.split('-');
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .for_each(|(a, b)| {
            nodes.entry(a).or_default().insert(b);
            nodes.entry(b).or_default().insert(a);
        });

    Network { nodes }
}

#[derive(Debug, Clone)]
struct Network<'a> {
    nodes: FxHashMap<&'a str, FxHashSet<&'a str>>,
}

impl<'a> Network<'a> {
    fn nodes(&self) -> impl Iterator<Item = &&'a str> {
        self.nodes.keys()
    }

    fn is_neighbor_of(&self, a: &str, b: &str) -> bool {
        self.nodes.get(a).map(|n| n.contains(b)).unwrap_or(false)
    }

    fn neighbors_of(&self, a: &str) -> Option<impl Iterator<Item = &&'a str>> {
        self.nodes.get(a).map(|a| a.iter())
    }

    fn longest_network(&'a self, start: &'a str) -> Vec<&'a str> {
        let mut queue = vec![vec![start]];
        let mut visited = FxHashSet::default();

        let mut longest_network = vec![];

        while let Some(network) = queue.pop() {
            let head = network.last().unwrap();

            if !visited.insert(*head) {
                continue;
            }

            let head_neighbors = self.nodes.get(head).unwrap();

            if network[..network.len() - 1]
                .iter()
                .all(|other| head_neighbors.contains(other))
            {
                if longest_network.len() < network.len() {
                    longest_network = network.clone();
                }

                if let Some(neighbors) = self.nodes.get(head) {
                    neighbors.iter().for_each(|neighbor| {
                        if !network.contains(neighbor) {
                            let mut network = network.clone();
                            network.push(*neighbor);
                            queue.push(network);
                        }
                    });
                }
            }
        }

        longest_network.sort();

        longest_network
    }
}

pub fn step1(s: &str) -> Answer {
    let network = parse(s);
    let mut triplets = FxHashSet::default();

    for node in network.nodes() {
        for neighbor in network.neighbors_of(node).unwrap() {
            if let Some(indirect_neighbors) = network.neighbors_of(neighbor) {
                for indirect_neighbor in indirect_neighbors {
                    if node != indirect_neighbor
                        && (node.starts_with('t')
                            || neighbor.starts_with('t')
                            || indirect_neighbor.starts_with('t'))
                        && network.is_neighbor_of(node, indirect_neighbor)
                    {
                        let mut triplet = [*node, *neighbor, *indirect_neighbor];
                        triplet.sort();
                        triplets.insert((triplet[0], triplet[1], triplet[2]));
                    }
                }
            }
        }
    }

    triplets.len().into()
}

pub fn step2(s: &str) -> Answer {
    let network = parse(s);

    let longest_network = network.nodes().fold(vec![], |acc, node| {
        let network = network.longest_network(node);

        if network.len() > acc.len() {
            network
        } else {
            acc
        }
    });

    longest_network.join(",").into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    #[test]
    fn step2_finds_correct_example_answer() {
        assert_eq!(step2(INPUT), Answer::Text("co,de,ka,ta".to_string()));
    }
}
