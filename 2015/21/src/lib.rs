use common::Answer;
use itertools::Itertools;

type IntType = u32;

#[derive(Debug, Clone, Copy)]
struct Item {
    cost: IntType,
    damage: IntType,
    armor: IntType,
}

static WEAPONS: [Item; 5] = [
    Item {
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

static ARMOR: [Item; 6] = [
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

static RINGS: [Item; 7] = [
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

fn rounds(hp: IntType, dmg: IntType, armor: IntType) -> IntType {
    let dmg = dmg.saturating_sub(armor).max(1);
    (hp as f64 / dmg as f64).ceil() as IntType
}

fn gold_per_battle(
    boss: (IntType, IntType, IntType),
    player_hp: IntType,
    cmp: fn(&IntType, &IntType) -> bool,
) -> impl Iterator<Item = IntType> {
    WEAPONS.iter().flat_map(move |weapon| {
        ARMOR.iter().flat_map(move |armor| {
            RINGS.iter().combinations(2).filter_map(move |rings| {
                let dmg = weapon.damage + rings[0].damage + rings[1].damage;
                let defence = armor.armor + rings[0].armor + rings[1].armor;

                if cmp(
                    &rounds(player_hp, boss.1, defence),
                    &rounds(boss.0, dmg, boss.2),
                ) {
                    Some(weapon.cost + armor.cost + rings[0].cost + rings[1].cost)
                } else {
                    None
                }
            })
        })
    })
}

pub fn step1(_: &str) -> Answer {
    gold_per_battle((104, 8, 1), 100, IntType::ge)
        .min()
        .unwrap()
        .into()
}

pub fn step2(_: &str) -> Answer {
    gold_per_battle((104, 8, 1), 100, IntType::lt)
        .max()
        .unwrap()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(
        input1 = { 8, 12 },
        input2 = { 7, 5 },
        input3 = { 5, 2 },
        result = { 4, 4 },
    )]
    fn rounds_determines_correct_winner_example(
        input1: IntType,
        input2: IntType,
        input3: IntType,
        result: IntType,
    ) {
        assert_eq!(rounds(input1, input2, input3), result);
    }

    #[parameterized(
        input = {(100, 10, 500), (100, 10, 10), (100, 10, 0)},
        output = {100, 100, 10},
    )]
    fn rounds_handles_edge_cases(input: (IntType, IntType, IntType), output: IntType) {
        assert_eq!(rounds(input.0, input.1, input.2), output);
    }
}
