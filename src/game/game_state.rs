use graphics::{
    color::{GREEN, RED},
    types::ColorComponent,
};
use rand::{prelude::SliceRandom, thread_rng, Rng};
use crate::constants::HERBIVORE_SIZE;

use super::{
    plant::Plant,
    species::{Direction, Herbivore},
};

pub struct GameState {
    pub herbivores: Vec<Option<Herbivore>>,
    pub plants: Vec<Plant>,
}

impl GameState {}

pub fn create_gamestate(n_herbivores: usize, n_plants: usize) -> GameState {
    let mut herbivores: Vec<Option<Herbivore>> = Vec::new();
    for _ in 0..n_herbivores {
        let mut herbivore = Some(create_herbivore());
        herbivores.push(herbivore);
    }

    let mut plants: Vec<Plant> = Vec::new();
    for _ in 0..n_plants {
        let mut plant = create_plant();
        plants.push(plant);
    }

    GameState {
        herbivores: herbivores,
        plants: plants,
    }
}


pub fn create_herbivore() -> Herbivore {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0..800.0);
    let y = rng.gen_range(0.0..800.0);
    let direction = rng.gen_range(0..360);

    Herbivore::new(HERBIVORE_SIZE, RED, x, y, direction, 5000, 0, 1.0)
}

pub fn create_plant() -> Plant {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0..800.0) as f64;
    let y = rng.gen_range(0.0..800.0) as f64;

    Plant::new(5.0, GREEN, x, y, true, 0, 3)
}
