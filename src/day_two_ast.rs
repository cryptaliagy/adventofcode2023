use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ColorEnum {
    Red,
    Blue,
    Green,
}

#[derive(Clone)]
pub struct Cube(pub u32, pub ColorEnum);

pub enum RoundEnum {
    Final(Cube),
    Chain(Cube, Box<RoundEnum>),
}

pub struct Round {
    cubes: HashMap<ColorEnum, u32>,
}

impl Round {
    pub fn new(round: RoundEnum) -> Round {
        Round::from_cubes(round.to_vec())
    }

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

impl RoundEnum {
    pub fn to_round(self) -> Round {
        match self {
            RoundEnum::Final(cube) => Round {
                cubes: HashMap::from_iter([(cube.1, cube.0)]),
            },
            RoundEnum::Chain(cube, round) => {
                let mut vec = vec![cube];
                vec.append(&mut round.as_ref().to_vec());
                Round {
                    cubes: vec.into_iter().map(|cube| (cube.1, cube.0)).collect(),
                }
            }
        }
    }

    pub fn to_vec(&self) -> Vec<Cube> {
        match self {
            RoundEnum::Final(cube) => vec![cube.clone()],
            RoundEnum::Chain(cube, round) => {
                let mut vec = vec![cube.clone()];
                vec.append(&mut round.to_vec());
                vec
            }
        }
    }
}

pub enum RoundsEnum {
    Final(Round),
    Chain(Round, Box<RoundsEnum>),
}

impl RoundsEnum {
    pub fn to_rounds(self) -> Vec<Round> {
        match self {
            RoundsEnum::Final(round) => vec![round],
            RoundsEnum::Chain(round, rounds) => {
                let mut vec = vec![round];
                vec.append(&mut rounds.to_rounds());
                vec
            }
        }
    }
}

pub struct Game {
    pub id: u32,
    rounds: Vec<Round>,
}

impl Game {
    pub fn new(id: u32, rounds: RoundsEnum) -> Game {
        Game {
            id,
            rounds: rounds.to_rounds(),
        }
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
