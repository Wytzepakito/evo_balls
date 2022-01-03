use graphics::{color::RED, types::ColorComponent};
use rand::{prelude::SliceRandom, thread_rng, Rng};

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

pub struct Circle {
    pub size: f64,
    pub color: [ColorComponent; 4],
    pub x: f64,
    pub y: f64,
    pub direction: Direction,
}

impl Circle {
    pub fn new(
        size: f64,
        color: [ColorComponent; 4],
        x: f64,
        y: f64,
        direction: Direction,
    ) -> Circle {
        Circle {
            size: size,
            color: color,
            x: x,
            y: y,
            direction: direction,
        }
    }

    pub fn advance(&mut self) {
        let mut new_x: f64;
        let mut new_y: f64;
        let (new_x_, new_y_) = self.get_move();

        new_x = new_x_;
        new_y = new_y_;


        // In case the new position is out of bounds.
        if new_x > 750.0 || new_x < 0.0 || new_y > 750.0 || new_y < 0.0 {
            let dir = Circle::get_new_direction(new_x, new_y);
            self.direction = dir;
            
            let (new_x_, new_y_) = self.get_move();
            new_x = new_x_;
            new_y = new_y_;
        }


        self.x = new_x;
        self.y = new_y;
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

    fn get_new_direction(new_x: f64, new_y: f64) -> Direction {
        if new_x > 750.0 {
            if new_y > 750.0 {
                return Direction::NorthEast;
            } else if new_y < 0.0 {
                return Direction::SouthEast;
            } else {
                let choices = [Direction::NorthEast, Direction::East, Direction::SouthEast];
                return choices
                    .choose(&mut rand::thread_rng())
                    .cloned()
                    .expect("Could not choose a new direction");
            }
        } else if new_x < 0.0 {
            if new_y > 750.0 {
                return Direction::NorthWest;
            } else if new_y < 0.0 {
                return Direction::SouthWest;
            } else {
                let choices = [Direction::NorthWest, Direction::West, Direction::SouthWest];
                return choices
                    .choose(&mut rand::thread_rng())
                    .cloned()
                    .expect("Could not choose a new direction");
            }
        } else if new_y > 750.0 {
            let choices = [Direction::NorthWest, Direction::North, Direction::NorthEast];
            return choices
                .choose(&mut rand::thread_rng())
                .cloned()
                .expect("Could not choose a new direction");
        } else {
            let choices = [Direction::SouthWest, Direction::South, Direction::SouthEast];
            return choices
                .choose(&mut rand::thread_rng())
                .cloned()
                .expect("Could not choose a new direction");
        }
    }
}

pub struct GameState {
    pub circles: Vec<Circle>,
}

impl GameState {}
