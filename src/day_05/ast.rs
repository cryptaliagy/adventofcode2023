use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Mapping(usize, usize, usize);

impl Mapping {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self(a, b, c)
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    pub source: String,
    pub target: String,
    pub mappings: Vec<Mapping>,
}

impl Map {
    pub fn map(&self, input: usize) -> usize {
        for Mapping(target, source, range) in &self.mappings {
            if *source <= input && input <= *source + *range {
                return target + (input - source);
            }
        }

        input
    }
}

#[derive(Debug)]
pub struct Puzzle {
    pub seeds: Vec<usize>,
    pub maps: Vec<Map>,
    pub source_maps: HashMap<String, Map>,
    pub target_maps: HashMap<String, Map>,
}

#[derive(Debug)]
pub struct SeedPair(pub usize, pub usize);

#[derive(Debug)]
pub struct Puzzle2 {
    pub seeds: Vec<SeedPair>,
    pub maps: Vec<Map>,
    pub source_maps: HashMap<String, Map>,
    pub target_maps: HashMap<String, Map>,
}

impl Puzzle {
    pub fn new(seeds: Vec<usize>, maps: Vec<Map>) -> Self {
        let mut source_maps = HashMap::new();
        let mut target_maps = HashMap::new();

        for map in maps.iter() {
            source_maps.insert(map.source.clone(), map.clone());
            target_maps.insert(map.target.clone(), map.clone());
        }

        Self {
            seeds,
            maps,
            source_maps,
            target_maps,
        }
    }

    pub fn map(&self, input: usize) -> usize {
        let mut label = "seed".to_string();
        let mut output = input;

        while label != "location" {
            let map = self.source_maps.get(&label).unwrap();
            label = map.target.clone();

            output = map.map(output);
        }

        output
    }

    pub fn find_closest_location(&self) -> usize {
        self.seeds.iter().map(|&seed| self.map(seed)).min().unwrap()
    }
}

impl Puzzle2 {
    pub fn new(seeds: Vec<SeedPair>, maps: Vec<Map>) -> Self {
        let mut source_maps = HashMap::new();
        let mut target_maps = HashMap::new();

        for map in maps.iter() {
            source_maps.insert(map.source.clone(), map.clone());
            target_maps.insert(map.target.clone(), map.clone());
        }

        Self {
            seeds,
            maps,
            source_maps,
            target_maps,
        }
    }

    pub fn map(&self, input: usize) -> usize {
        let mut label = "seed".to_string();
        let mut output = input;

        while label != "location" {
            let map = self.source_maps.get(&label).unwrap();
            label = map.target.clone();

            output = map.map(output);
        }

        output
    }

    pub fn find_closest_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|SeedPair(a, b)| (*a..(*a + *b)).map(|x| self.map(x)).min().unwrap())
            .min()
            .unwrap()
    }
}
