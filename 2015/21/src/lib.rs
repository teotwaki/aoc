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
) -> Vec<IntType> {
    let (boss_hp, boss_dmg, boss_armor) = boss;
    let mut budgets = vec![];

    for weapon in WEAPONS.iter() {
        for armor in ARMOR.iter() {
            for rings in RINGS.iter().combinations(2) {
                let dmg = weapon.damage + rings[0].damage + rings[1].damage;
                let defence = armor.armor + rings[0].armor + rings[1].armor;

                if cmp(
                    &rounds(player_hp, boss_dmg, defence),
                    &rounds(boss_hp, dmg, boss_armor),
                ) {
                    budgets.push(weapon.cost + armor.cost + rings[0].cost + rings[1].cost);
                }
            }
        }
    }

    budgets
}

pub fn step1(_: &str) -> Answer {
    gold_per_battle((104, 8, 1), 100, IntType::ge)
        .iter()
        .min()
        .copied()
        .unwrap()
        .into()
}

pub fn step2(_: &str) -> Answer {
    gold_per_battle((104, 8, 1), 100, IntType::lt)
        .iter()
        .max()
        .copied()
        .unwrap()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rounds_determines_correct_winner_example() {
        assert_eq!(rounds(8, 7, 5), 4);
        assert_eq!(rounds(12, 5, 2), 4);
    }

    #[test]
    fn rounds_handles_edge_cases() {
        assert_eq!(rounds(100, 10, 500), 100);
        assert_eq!(rounds(100, 10, 10), 100);
        assert_eq!(rounds(100, 10, 0), 10);
    }
}
