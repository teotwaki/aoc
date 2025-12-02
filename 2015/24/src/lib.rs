use common::Answer;
use itertools::Itertools;

type IntType = u64;

fn parse(s: &str) -> Vec<IntType> {
    s.lines().map(|l| l.parse::<IntType>().unwrap()).collect()
}

fn can_split_into_groups(weights: &[IntType], target_weight: IntType, groups: IntType) -> bool {
    if groups == 1 {
        return weights.iter().sum::<IntType>() == target_weight;
    }

    for i in 2..=weights.len() {
        for combination in weights.iter().combinations(i) {
            if combination.iter().copied().sum::<IntType>() == target_weight {
                let remaining: Vec<IntType> = weights
                    .iter()
                    .filter(|w| !combination.contains(w))
                    .copied()
                    .collect();

                if can_split_into_groups(&remaining, target_weight, groups - 1) {
                    return true;
                }
            }
        }
    }

    false
}

fn find_grouping(s: &str, groups: IntType) -> Answer {
    let weights = parse(s);
    let total_weight: IntType = weights.iter().sum();
    let target_weight = total_weight / groups;

    for i in 2..=weights.len() {
        let mut min_qe = IntType::MAX;

        for combination in weights.iter().combinations(i) {
            if combination.iter().copied().sum::<IntType>() == target_weight {
                let qe = combination.iter().copied().product();
                if qe < min_qe {
                    let remaining: Vec<IntType> = weights
                        .iter()
                        .filter(|w| !combination.contains(w))
                        .copied()
                        .collect();

                    if can_split_into_groups(&remaining, target_weight, groups - 1) {
                        min_qe = qe;
                    }
                }
            }
        }

        if min_qe != u64::MAX {
            return min_qe.into();
        }
    }

    ().into()
}

pub fn step1(s: &str) -> Answer {
    find_grouping(s, 3)
}

pub fn step2(s: &str) -> Answer {
    find_grouping(s, 4)
}
