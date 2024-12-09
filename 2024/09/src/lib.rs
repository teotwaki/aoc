use common::Answer;

type IntType = u16;
type FileSize = u8;

#[derive(Debug, Clone, Copy)]
enum Block {
    File(IntType),
    FreeSpace,
}

fn parse_blocks(s: &str) -> Vec<Block> {
    let mut id = 0;

    s.lines()
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as FileSize))
        .enumerate()
        .flat_map(|(i, c)| {
            if i % 2 == 0 {
                let v = vec![Block::File(id); c as usize];
                id += 1;

                v
            } else {
                vec![Block::FreeSpace; c as usize]
            }
        })
        .collect()
}

fn checksum(blocks: &[Block]) -> usize {
    blocks
        .iter()
        .enumerate()
        .map(|(i, b)| match b {
            Block::File(n) => i * *n as usize,
            _ => 0,
        })
        .sum::<usize>()
}

pub fn step1(s: &str) -> Answer {
    let mut blocks = parse_blocks(s);

    let mut i = 0;
    let mut j = blocks.len() - 1;

    while i < j {
        if let Block::File(_) = blocks[i] {
            i += 1;
            continue;
        }

        if let Block::FreeSpace = blocks[j] {
            j -= 1;
            continue;
        }

        blocks[i] = blocks[j];
        blocks[j] = Block::FreeSpace;

        i += 1;
        j -= 1;
    }

    checksum(&blocks).into()
}

#[derive(Debug, Clone, Copy)]
enum Object {
    File(FileSize, IntType),
    FreeSpace(FileSize),
}

impl From<Object> for Vec<Block> {
    fn from(value: Object) -> Self {
        use Object::*;

        match value {
            File(len, id) => (0..len).map(|_| Block::File(id)).collect(),
            FreeSpace(len) => (0..len).map(|_| Block::FreeSpace).collect(),
        }
    }
}

fn parse_objects(s: &str) -> Vec<Object> {
    let mut id = 0;

    s.lines()
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as FileSize))
        .enumerate()
        .filter_map(|(i, c)| {
            if i % 2 == 0 {
                let o = Object::File(c, id as IntType);
                id += 1;

                Some(o)
            } else if c != 0 {
                Some(Object::FreeSpace(c))
            } else {
                None
            }
        })
        .collect()
}

pub fn step2(s: &str) -> Answer {
    let mut objects = parse_objects(s);

    let mut j = objects.len() - 1;

    while j != 0 {
        let Object::File(file_size, _) = objects[j] else {
            j -= 1;
            continue;
        };

        let mut i = 1;
        while i < j {
            let Object::FreeSpace(free_space) = objects[i] else {
                i += 1;
                continue;
            };

            if free_space < file_size {
                i += 1;
                continue;
            }

            let diff = free_space - file_size;

            if diff == 0 {
                objects[i] = objects[j];
                objects[j] = Object::FreeSpace(free_space);
            } else {
                objects[i] = Object::FreeSpace(diff);
                objects.insert(i, objects[j]);
                j += 1;
                objects[j] = Object::FreeSpace(file_size);
            }

            break;
        }

        j -= 1;
    }

    let blocks = objects
        .iter()
        .flat_map(|o| Vec::from(*o))
        .collect::<Vec<_>>();

    checksum(&blocks).into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"2333133121414131402"#;

    #[test]
    fn step1_finds_correct_example_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(1928));
    }

    #[test]
    fn step2_finds_correct_example_answer() {
        assert_eq!(step2(INPUT), Answer::Unsigned(2858));
    }
}
