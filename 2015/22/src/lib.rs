use common::Answer;
use std::{cmp::Ordering, collections::BinaryHeap};

type IntType = i16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Character {
    hp: IntType,
    mana: IntType,
    dmg: IntType,
    armor: IntType,
}

impl Character {
    fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    fn can_afford(&self, spell: &Spell) -> bool {
        self.mana >= spell.cost
    }
}

static BOSS: Character = Character {
    hp: 55,
    dmg: 8,
    mana: 0,
    armor: 0,
};

static PLAYER: Character = Character {
    hp: 50,
    mana: 500,
    dmg: 0,
    armor: 0,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Modifier {
    player_armor: IntType,
    player_mana: IntType,
    boss_dmg: IntType,
    player_dmg: bool,
}

impl Modifier {
    fn new() -> Self {
        Self {
            player_armor: 0,
            player_mana: 0,
            boss_dmg: 0,
            player_dmg: false,
        }
    }

    fn hard() -> Self {
        Self {
            player_armor: 0,
            player_mana: 0,
            boss_dmg: 0,
            player_dmg: true,
        }
    }

    fn is_active(&self, effect: Effect) -> bool {
        use Effect::*;

        match effect {
            Armor => self.player_armor != 0,
            Poison => self.boss_dmg != 0,
            Recharge => self.player_mana != 0,
        }
    }

    fn cast(&mut self, effect: Effect) {
        use Effect::*;

        match effect {
            Armor => {
                assert_eq!(self.player_armor, 0);
                self.player_armor = 6;
            }
            Poison => {
                assert_eq!(self.boss_dmg, 0);
                self.boss_dmg = 6;
            }
            Recharge => {
                assert_eq!(self.player_mana, 0);
                self.player_mana = 5;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Effect {
    Armor,
    Poison,
    Recharge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Spell {
    cost: IntType,
    damage: IntType,
    health: IntType,
    effect: Option<Effect>,
}

static SPELLS: [Spell; 5] = [
    // Magic Missile
    Spell {
        cost: 53,
        damage: 4,
        health: 0,
        effect: None,
    },
    // Drain
    Spell {
        cost: 73,
        damage: 2,
        health: 2,
        effect: None,
    },
    // Shield
    Spell {
        cost: 113,
        effect: Some(Effect::Armor),
        health: 0,
        damage: 0,
    },
    // Poison
    Spell {
        cost: 173,
        effect: Some(Effect::Poison),
        health: 0,
        damage: 0,
    },
    // Recharge
    Spell {
        cost: 229,
        effect: Some(Effect::Recharge),
        health: 0,
        damage: 0,
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct GameState {
    modifier: Modifier,
    player: Character,
    boss: Character,
    cost: IntType,
    player_turn: bool,
    history: Vec<(Spell, Character, Character)>,
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl GameState {
    fn new(player: Character, boss: Character, modifier: Modifier) -> Self {
        Self {
            player,
            boss,
            modifier,
            cost: 0,
            player_turn: true,
            history: vec![],
        }
    }

    fn is_finished(&self) -> bool {
        self.player.is_dead() || self.boss.is_dead()
    }

    fn is_victory(&self) -> bool {
        !self.player.is_dead() && self.boss.is_dead()
    }

    #[must_use]
    fn next(&mut self) -> Vec<Self> {
        self.player_turn = !self.player_turn;
        self.apply_effects();

        if !self.player_turn {
            SPELLS
                .iter()
                .filter(|spell| self.player.can_afford(spell))
                .filter(|spell| {
                    if let Some(effect) = spell.effect {
                        !self.modifier.is_active(effect)
                    } else {
                        true
                    }
                })
                .map(|spell| {
                    let mut state = self.clone();
                    if !state.is_finished() {
                        state.player_turn(spell);
                    }

                    state
                })
                .collect()
        } else {
            let mut state = self.clone();
            if !self.is_finished() {
                state.boss_turn();
            }

            vec![state]
        }
    }

    fn apply_effects(&mut self) {
        if self.modifier.boss_dmg != 0 {
            self.boss.hp -= 3;
            self.modifier.boss_dmg -= 1;
        }

        if self.modifier.player_mana != 0 {
            self.player.mana += 101;
            self.modifier.player_mana -= 1;
        }

        if self.modifier.player_armor != 0 {
            self.player.armor = 7;
            self.modifier.player_armor -= 1;
        } else {
            self.player.armor = 0;
        }

        if !self.player_turn && self.modifier.player_dmg {
            self.player.hp -= 1;
        }
    }

    fn player_turn(&mut self, spell: &Spell) {
        if let Some(effect) = spell.effect {
            self.modifier.cast(effect);
        }

        self.boss.hp -= spell.damage;
        self.player.hp += spell.health;
        self.player.mana -= spell.cost;
        self.cost += spell.cost;

        self.history.push((*spell, self.player, self.boss));
    }

    fn boss_turn(&mut self) {
        if self.boss.hp > 0 {
            self.player.hp -= (self.boss.dmg - self.player.armor).max(1);
        }
    }
}

fn dijkstra(start: GameState) -> Option<IntType> {
    let mut heap = BinaryHeap::new();

    heap.push(start);

    while let Some(mut state) = heap.pop() {
        if state.is_victory() {
            return Some(state.cost);
        }

        if state.is_finished() {
            continue;
        }

        for state in state.next() {
            heap.push(state);
        }
    }

    None
}

pub fn step1(_: &str) -> Answer {
    let state = GameState::new(PLAYER, BOSS, Modifier::new());
    dijkstra(state).unwrap().into()
}

pub fn step2(_: &str) -> Answer {
    let state = GameState::new(PLAYER, BOSS, Modifier::hard());
    dijkstra(state).unwrap().into()
}

#[cfg(test)]
mod test_2015_22 {
    use super::*;

    #[test]
    fn dijkstra_finds_correct_answer_example1() {
        let player = Character {
            hp: 10,
            mana: 250,
            dmg: 0,
            armor: 0,
        };

        let boss = Character {
            hp: 13,
            mana: 0,
            dmg: 8,
            armor: 0,
        };

        let state = GameState::new(player, boss, Modifier::new());
        assert_eq!(dijkstra(state), Some(226));
    }

    #[test]
    fn dijkstra_finds_correct_answer_example2() {
        let player = Character {
            hp: 10,
            mana: 250,
            dmg: 0,
            armor: 0,
        };

        let boss = Character {
            hp: 14,
            mana: 0,
            dmg: 8,
            armor: 0,
        };

        let state = GameState::new(player, boss, Modifier::new());
        assert_eq!(dijkstra(state), Some(641));
    }
}
