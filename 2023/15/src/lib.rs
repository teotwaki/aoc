use common::Answer;

fn hash(s: &str) -> u8 {
    s.chars()
        .map(|c| c as u32)
        .fold(0u32, |acc, x| ((acc + x) * 17) % 256) as u8
}

#[derive(Debug)]
enum Step<'a> {
    Remove(&'a str, u8),
    Add(&'a str, u8, u8),
}

impl<'a> From<&'a str> for Step<'a> {
    fn from(s: &'a str) -> Self {
        if s.ends_with('-') {
            let label = &s[0..(s.len() - 1)];
            Self::Remove(label, hash(label))
        } else {
            let parts: Vec<_> = s.split('=').collect();
            let focal_length = parts[1].parse::<u8>().unwrap();
            Self::Add(parts[0], hash(parts[0]), focal_length)
        }
    }
}

impl<'a> Step<'a> {
    fn run(&'a self, boxes: &mut [Vec<Lens<'a>>]) {
        use Step::*;

        match self {
            Remove(label, target_box) => {
                if let Some(pos) = boxes[*target_box as usize]
                    .iter()
                    .position(|x| &x.label == label)
                {
                    boxes[*target_box as usize].remove(pos);
                }
            }
            Add(label, target_box, focal_length) => {
                if let Some(pos) = boxes[*target_box as usize]
                    .iter()
                    .position(|x| &x.label == label)
                {
                    boxes[*target_box as usize][pos].focal_length = *focal_length;
                } else {
                    boxes[*target_box as usize].push(Lens {
                        label,
                        focal_length: *focal_length,
                    })
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

pub fn step1(s: &str) -> Answer {
    s.lines()
        .flat_map(|l| l.split(','))
        .map(|s| hash(s) as usize)
        .sum::<usize>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let steps: Vec<_> = s
        .lines()
        .flat_map(|l| l.split(','))
        .map(Step::from)
        .collect();

    let mut boxes = vec![vec![]; 256];
    steps.iter().for_each(|s| s.run(&mut boxes));

    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, l)| (i + 1) * (j + 1) * l.focal_length as usize)
                .sum::<usize>()
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_works() {
        assert_eq!(hash("HASH"), 52);
    }
}
