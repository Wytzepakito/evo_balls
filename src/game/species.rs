use graphics::types::ColorComponent;
use rand::prelude::SliceRandom;

use crate::constants::HERBIVORE_UPPER_BOUND;

use super::plant::Plant;

#[derive(Debug, Clone)]
pub enum Direction {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

pub struct Herbivore {
    pub size: f64,
    pub color: [ColorComponent; 4],
    pub x: f64,
    pub y: f64,
    pub direction: Direction,
    pub energy_level: u32,
    pub alive_ticks: u32,
}

impl Herbivore {
    pub fn new(
        size: f64,
        color: [ColorComponent; 4],
        x: f64,
        y: f64,
        direction: Direction,
        energy_level: u32,
        alive_ticks: u32,
    ) -> Herbivore {
        Herbivore {
            size: size,
            color: color,
            x: x,
            y: y,
            direction: direction,
            energy_level: energy_level,
            alive_ticks: alive_ticks,
        }
    }

    pub fn advance(&mut self) {
        let mut new_x: f64;
        let mut new_y: f64;
        let (new_x_, new_y_) = self.get_move();

        new_x = new_x_;
        new_y = new_y_;

        // In case the new position is out of bounds.
        if new_x > 770.0 || new_x < 0.0 || new_y > 770.0 || new_y < 0.0 {
            let dir = Herbivore::get_new_direction(&self, new_x, new_y);
            self.direction = dir;

            let (new_x_, new_y_) = self.get_move();
            new_x = new_x_;
            new_y = new_y_;
        }

        self.x = new_x;
        self.y = new_y;
        self.energy_level -= 1;
    }

    fn get_move(&mut self) -> (f64, f64) {
        let mut new_x: f64;
        let mut new_y: f64;

        match self.direction {
            Direction::North => {
                new_x = self.x;
                new_y = self.y - 1.0;
            }
            Direction::NorthWest => {
                new_x = self.x + 1.0;
                new_y = self.y - 1.0;
            }
            Direction::West => {
                new_x = self.x + 1.0;
                new_y = self.y;
            }
            Direction::SouthWest => {
                new_x = self.x + 1.0;
                new_y = self.y + 1.0;
            }
            Direction::South => {
                new_x = self.x;
                new_y = self.y + 1.0;
            }
            Direction::SouthEast => {
                new_x = self.x - 1.0;
                new_y = self.y + 1.0;
            }
            Direction::East => {
                new_x = self.x - 1.0;
                new_y = self.y;
            }
            Direction::NorthEast => {
                new_x = self.x - 1.0;
                new_y = self.y - 1.0;
            }
            _ => unimplemented!(),
        }
        (new_x, new_y)
    }







    fn get_new_direction(&self, new_x: f64, new_y: f64) -> Direction {
        // Upper right corner
        if new_x > HERBIVORE_UPPER_BOUND && new_y > HERBIVORE_UPPER_BOUND {
            return Direction::NorthEast;
        } // Lower right corner
        else if ( new_x > HERBIVORE_UPPER_BOUND && new_y  < 0.0) {
            return Direction::SouthEast;
        } // Lower left corner
        else if ( new_x < 0.0 && new_y > HERBIVORE_UPPER_BOUND) {
            return Direction::NorthWest;
        } // Upper left corner
        else if ( new_x < 0.0 && new_y > HERBIVORE_UPPER_BOUND) {
            return Direction::SouthWest;
        }

        if (new_x > HERBIVORE_UPPER_BOUND) {
            return self.choose_east_options();
        } else if (new_x < 0.0) {
            return self.choose_west_options();
        } else if (new_y > HERBIVORE_UPPER_BOUND) {
            return self.choose_north_options();
        } else  {
            return self.choose_south_options();
        }
    }




    fn choose_east_options(&self) -> Direction {
        let eastward_choices = [Direction::NorthEast, Direction::East, Direction::SouthEast];
        return eastward_choices
                    .choose(&mut rand::thread_rng())
                    .cloned()
                    .expect("Could not choose a new direction");
    }
    fn choose_west_options(&self) -> Direction {
        let westward_choices = [Direction::NorthWest, Direction::West, Direction::SouthWest];
        return westward_choices
                    .choose(&mut rand::thread_rng())
                    .cloned()
                    .expect("Could not choose a new direction");
    }

    fn choose_north_options(&self) -> Direction {
        let northward_choices = [Direction::NorthEast, Direction::North, Direction::NorthWest];
        return northward_choices
                    .choose(&mut rand::thread_rng())
                    .cloned()
                    .expect("Could not choose a new direction");
    }

    fn choose_south_options(&self) -> Direction {
        let eastward_choices = [Direction::SouthEast, Direction::South, Direction::SouthWest];
        return eastward_choices
                    .choose(&mut rand::thread_rng())
                    .cloned()
                    .expect("Could not choose a new direction");
    }



    pub fn try_eat(&mut self, plant: &mut Plant) {
        //println!("plant coordinates x: {} y: {}", plant.x, plant.y);
        //println!("Should be inside these x: {}, {} y: {}, {}", self.x - (self.size / 2.0), self.x + (self.size / 2.0), self.y - (self.size / 2.0), self.y + (self.size / 2.0));

        if (plant.x - (plant.size / 2.0)) > self.x 
            && (plant.x + (plant.size / 2.0)) < (self.x + self.size )
            && (plant.y - ( plant.size / 2.0)) > (self.y)
            && (plant.y + (plant.size / 2.0)) < (self.y + self.size)
        {
            self.energy_level += plant.energy;
            plant.alive = false;
        }
    }
}
