use common::Answer;
use rustc_hash::FxHashMap;
use std::{cell::RefCell, fmt::Display, rc::Rc};

type NodeList = Vec<Rc<RefCell<Node>>>;
type IntType = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Id(IntType);

impl TryFrom<&str> for Id {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() > 4 {
            Err("ID string too long")
        } else {
            let v = s
                .char_indices()
                .map(|(i, c)| (i, c as IntType))
                .fold(0, |acc, (i, v)| acc + (v << (i * 8)));

            Ok(Id(v))
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut remain = self.0;
        let mut s = String::new();

        while remain != 0 {
            s.insert(0, (remain & 0xff) as u8 as char);
            remain >>= 8;
        }

        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
struct Node {
    id: Id,
    edges: NodeList,
}

impl Node {
    fn new(id: Id) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            id,
            edges: Vec::new(),
        }))
    }

    fn add_vertex(&mut self, node: Rc<RefCell<Node>>) {
        self.edges.push(node);
    }

    fn count_paths_to_inner(
        &self,
        target: Id,
        memo: &mut rustc_hash::FxHashMap<Id, usize>,
    ) -> usize {
        if self.id == target {
            return 1;
        }

        if let Some(&cached) = memo.get(&self.id) {
            return cached;
        }

        let mut path_count = 0;

        for edge in &self.edges {
            let node = edge.borrow();
            path_count += node.count_paths_to_inner(target, memo);
        }

        memo.insert(self.id, path_count);
        path_count
    }

    fn count_paths_to(&self, target: Id) -> usize {
        self.count_paths_to_inner(target, &mut FxHashMap::default())
    }

    fn count_paths_to_with_required_inner(
        &self,
        target: Id,
        required1: Id,
        required2: Id,
        mut seen1: bool,
        mut seen2: bool,
        memo: &mut rustc_hash::FxHashMap<(Id, bool, bool), usize>,
    ) -> usize {
        if self.id == required1 {
            seen1 = true;
        } else if self.id == required2 {
            seen2 = true;
        } else if self.id == target {
            return (seen1 && seen2) as usize;
        }

        let key = (self.id, seen1, seen2);

        if let Some(&cached) = memo.get(&key) {
            return cached;
        }

        let mut path_count = 0;

        for edge in &self.edges {
            let node = edge.borrow();
            path_count += node.count_paths_to_with_required_inner(
                target, required1, required2, seen1, seen2, memo,
            );
        }

        memo.insert(key, path_count);
        path_count
    }

    fn count_paths_to_with_required(&self, target: Id, required1: Id, required2: Id) -> usize {
        self.count_paths_to_with_required_inner(
            target,
            required1,
            required2,
            false,
            false,
            &mut FxHashMap::default(),
        )
    }
}

fn parse(s: &str) -> Vec<(Id, Id)> {
    s.lines()
        .flat_map(|l| {
            let mut parts = l.split(": ");
            let node = parts.next().unwrap();
            parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(move |dest| (Id::try_from(node).unwrap(), Id::try_from(dest).unwrap()))
        })
        .collect()
}

pub fn step1(s: &str) -> Answer {
    let you = Id::try_from("you").unwrap();
    let out = Id::try_from("out").unwrap();

    let pairs = parse(s);
    let nodes = {
        let mut nodes: NodeList = pairs.iter().map(|(id, _)| Node::new(*id)).collect();
        nodes.push(Node::new(out));

        nodes
    };

    fn find_node(id: Id, nodes: &[Rc<RefCell<Node>>]) -> &Rc<RefCell<Node>> {
        nodes.iter().find(|n| n.borrow().id == id).unwrap()
    }

    pairs.iter().for_each(|(src_id, dest_id)| {
        let src_node = find_node(*src_id, &nodes);
        let dest_node = find_node(*dest_id, &nodes);
        src_node.borrow_mut().add_vertex(dest_node.clone());
    });

    find_node(you, &nodes).borrow().count_paths_to(out).into()
}

pub fn step2(s: &str) -> Answer {
    let svr = Id::try_from("svr").unwrap();
    let out = Id::try_from("out").unwrap();
    let fft = Id::try_from("fft").unwrap();
    let dac = Id::try_from("dac").unwrap();

    let pairs = parse(s);
    let nodes = {
        let mut nodes: NodeList = pairs.iter().map(|(id, _)| Node::new(*id)).collect();
        nodes.push(Node::new(out));

        nodes
    };

    fn find_node(id: Id, nodes: &[Rc<RefCell<Node>>]) -> &Rc<RefCell<Node>> {
        nodes.iter().find(|n| n.borrow().id == id).unwrap()
    }

    pairs.iter().for_each(|(src_id, dest_id)| {
        let src_node = find_node(*src_id, &nodes);
        let dest_node = find_node(*dest_id, &nodes);
        src_node.borrow_mut().add_vertex(dest_node.clone());
    });

    find_node(svr, &nodes)
        .borrow()
        .count_paths_to_with_required(out, fft, dac)
        .into()
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    const INPUT: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

    #[parameterized(
        input = { "aaa", "bbb", "zzz" },
        result = { 6_381_921, 6_447_714, 8_026_746},

    )]
    fn id_try_from_handles_common_values(input: &str, result: IntType) {
        assert_eq!(Id::try_from(input), Ok(Id(result)));
    }

    #[test]
    fn id_try_from_bails_on_too_long_input() {
        assert!(Id::try_from("abcde").is_err());
    }

    #[parameterized(
        input = { 6_381_921, 6_447_714, 8_026_746},
        result = { "aaa", "bbb", "zzz" },
    )]
    fn id_to_string_produces_correct_output(input: IntType, result: &str) {
        assert_eq!(Id(input).to_string(), result);
    }

    #[test]
    fn parse_extracts_correct_number_of_lines() {
        assert_eq!(parse(INPUT).len(), 17);
    }

    #[test]
    fn step1_computes_expected_sample_result() {
        assert_eq!(step1(INPUT), Answer::Unsigned(5));
    }

    const INPUT2: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

    #[test]
    fn step2_computes_expected_sample_result() {
        assert_eq!(step2(INPUT2), Answer::Unsigned(2));
    }
}
