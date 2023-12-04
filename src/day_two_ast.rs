use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ColorEnum {
    Red,
    Blue,
    Green,
}

#[derive(Clone)]
pub struct Cube(pub u32, pub ColorEnum);

pub struct Round {
    cubes: HashMap<ColorEnum, u32>,
}

impl Round {
    pub fn from_cubes(cubes: Vec<Cube>) -> Round {
        Round {
            cubes: cubes.into_iter().map(|cube| (cube.1, cube.0)).collect(),
        }
    }

    pub fn is_possible_given(&self, maxima: &Round) -> bool {
        self.cubes
            .iter()
            .all(|(color, &count)| maxima.cubes[color] >= count)
    }
}

pub struct Game {
    pub id: u32,
    rounds: Vec<Round>,
}

impl Game {
    pub fn new(id: u32, rounds: Vec<Round>) -> Game {
        Game { id, rounds }
    }

    pub fn is_possible_given(&self, maxima: &Round) -> bool {
        self.rounds
            .iter()
            .all(|round| round.is_possible_given(maxima))
    }

    pub fn power(&self) -> u32 {
        let rounds = self
            .rounds
            .iter()
            .map(|round| &round.cubes)
            .collect::<Vec<_>>();

        let mut round_totals = HashMap::new();

        for round in rounds {
            for (color, &count) in round {
                round_totals
                    .entry(color)
                    .and_modify(|c| *c = std::cmp::max(*c, count))
                    .or_insert(count);
            }
        }

        round_totals.values().product()
    }
}
