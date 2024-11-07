use common::Answer;

fn is_visible_left(tree_map: &[i8], idx: usize, width: usize) -> bool {
    for i in ((idx - (idx % width))..(idx)).rev() {
        if tree_map[i] >= tree_map[idx] {
            return false;
        }
    }

    true
}

fn is_visible_right(tree_map: &[i8], idx: usize, width: usize) -> bool {
    for i in (idx + 1)..(idx + (width - (idx % width))) {
        if tree_map[i] >= tree_map[idx] {
            return false;
        }
    }

    true
}

fn is_visible_top(tree_map: &[i8], idx: usize, width: usize) -> bool {
    let mut i = idx;

    while i >= width {
        i -= width;
        if tree_map[i] >= tree_map[idx] {
            return false;
        }
    }

    true
}

fn is_visible_bottom(tree_map: &[i8], idx: usize, width: usize, height: usize) -> bool {
    let mut i = idx;

    while i < width * (height - 1) {
        i += width;
        if tree_map[i] >= tree_map[idx] {
            return false;
        }
    }

    true
}

fn is_visible(tree_map: &[i8], idx: usize, width: usize, height: usize) -> bool {
    is_visible_left(tree_map, idx, width)
        || is_visible_right(tree_map, idx, width)
        || is_visible_top(tree_map, idx, width)
        || is_visible_bottom(tree_map, idx, width, height)
}

fn scenic_score_left(tree_map: &[i8], idx: usize, width: usize) -> usize {
    let mut score = 0;

    for i in ((idx - (idx % width))..(idx)).rev() {
        score += 1;
        if tree_map[i] >= tree_map[idx] {
            break;
        }
    }

    score
}

fn scenic_score_right(tree_map: &[i8], idx: usize, width: usize) -> usize {
    let mut score = 0;

    for i in (idx + 1)..(idx + (width - (idx % width))) {
        score += 1;
        if tree_map[i] >= tree_map[idx] {
            break;
        }
    }

    score
}

fn scenic_score_top(tree_map: &[i8], idx: usize, width: usize) -> usize {
    let mut i = idx;
    let mut score = 0;

    while i >= width {
        score += 1;
        i -= width;
        if tree_map[i] >= tree_map[idx] {
            break;
        }
    }

    score
}

fn scenic_score_bottom(tree_map: &[i8], idx: usize, width: usize, height: usize) -> usize {
    let mut i = idx;
    let mut score = 0;

    while i < width * (height - 1) {
        score += 1;
        i += width;
        if tree_map[i] >= tree_map[idx] {
            break;
        }
    }

    score
}

fn calculate_scenic_score(tree_map: &[i8], idx: usize, width: usize, height: usize) -> usize {
    scenic_score_left(tree_map, idx, width)
        * scenic_score_right(tree_map, idx, width)
        * scenic_score_top(tree_map, idx, width)
        * scenic_score_bottom(tree_map, idx, width, height)
}

fn parse_tree_map(s: &str) -> Vec<Vec<i8>> {
    s.lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Couldn't parse tree into size"))
                .map(|i| i as i8)
                .collect::<Vec<i8>>()
        })
        .collect()
}

pub fn step1(s: &str) -> Answer {
    let tree_map = parse_tree_map(s);

    let width = tree_map[0].len();
    let height = tree_map.len();
    let mut visible_tree_count = width * 2 + height * 2 - 4;

    let tree_map: Vec<i8> = tree_map.into_iter().flatten().collect();

    for i in 1..(height - 1) {
        for j in 1..(width - 1) {
            if is_visible(&tree_map, (i * width) + j, width, height) {
                visible_tree_count += 1;
            }
        }
    }

    visible_tree_count.into()
}

pub fn step2(s: &str) -> Answer {
    let tree_map = parse_tree_map(s);
    let width = tree_map[0].len();
    let height = tree_map.len();

    let tree_map: Vec<i8> = tree_map.into_iter().flatten().collect();

    let max_scenic_score = tree_map
        .iter()
        .enumerate()
        .map(|(i, _)| calculate_scenic_score(&tree_map, i, width, height))
        .max()
        .expect("Couldn't find a maximum scenic score");

    max_scenic_score.into()
}
