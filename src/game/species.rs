use graphics::types::ColorComponent;
use rand::{prelude::SliceRandom, Rng};
use core::f32::consts::PI;

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
    pub x: f32,
    pub y: f32,
    pub direction: u8,
    pub energy_level: u32,
    pub alive_ticks: u32,
    pub speed: f32,
}

impl Herbivore {
    pub fn new(
        size: f64,
        color: [ColorComponent; 4],
        x: f32,
        y: f32,
        direction: u8,
        energy_level: u32,
        alive_ticks: u32,
        speed: f32,
    ) -> Herbivore {
        Herbivore {
            size: size,
            color: color,
            x: x,
            y: y,
            direction: direction,
            energy_level: energy_level,
            alive_ticks: alive_ticks,
            speed: speed,
        }
    }

    pub fn advance(&mut self) {

        let (new_x, new_y) = self.get_move();



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

    fn get_move(&mut self) -> (f32, f32) {



        
        let new_x: f32 = self.x +  self.speed * f32::cos(self.direction as f32 * ( PI / 180.0 ));
        let new_y: f32 = self.y + self.speed * f32::sin(self.direction as f32 * ( PI / 180.0 ));

        (new_x, new_y)
    }







    fn get_new_direction(&self, new_x: f32, new_y: f32) -> u8 {
        let mut rng = rand::thread_rng();
        // Upper right corner
        if new_x > HERBIVORE_UPPER_BOUND && new_y > HERBIVORE_UPPER_BOUND {
            return rng.gen_range(0..90);
        } // Lower right corner
        else if ( new_x > HERBIVORE_UPPER_BOUND && new_y  < 0.0) {
            return rng::get_range(90..180);
        } // Lower left corner
        else if ( new_x < 0.0 && new_y > HERBIVORE_UPPER_BOUND) {
            return rng::get_range(270..360);
        } // Upper left corner
        else if ( new_x < 0.0 && new_y > HERBIVORE_UPPER_BOUND) {
            return rng::get_range(180..270);
        }

        if (new_x > HERBIVORE_UPPER_BOUND) {
            return rng::get_range(0..180);
        } else if (new_x < 0.0) {
            return rng::get_range(180..360);
        } else if (new_y > HERBIVORE_UPPER_BOUND) {
            return self.choose_north_options();
        } else  {
            return rng::get_range(90..270);
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
