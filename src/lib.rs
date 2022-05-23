use game::{
    game_state::GameState,
    };

use graphics::{ ellipse, rectangle, Transformed, clear};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use rand::Rng;

pub mod game;
pub mod constants;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct App {
    pub gl: GlGraphics,
    gamestate: GameState,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::color::WHITE;

        // Clear the screen
        clear(WHITE, &mut self.gl);

        let mut context = self.gl.draw_begin(args.viewport());

        // DRAW PLANTS
        for plant in self.gamestate.plants.iter_mut() {
            if plant.alive {
                let c = context.trans(plant.x - plant.size, plant.y - plant.size);
                rectangle(
                    plant.color,
                    [plant.size, plant.size, plant.size, plant.size],
                    c.transform,
                    &mut self.gl,
                )
            } else if plant.dead_ticks == 1000 && plant.alive == false {
                plant.alive = true;
                plant.dead_ticks = 0;
            } else {
                plant.dead_ticks += 1;
            }
        }
        // DRAW HERBIVORES
        for herbivore_op in &self.gamestate.herbivores {
            if let Some(herbivore) = herbivore_op {
                let c = context.trans(herbivore.x - herbivore.size, herbivore.y - herbivore.size);
                ellipse(
                    herbivore.color,
                    [
                        herbivore.size,
                        herbivore.size,
                        herbivore.size,
                        herbivore.size,
                    ],
                    c.transform,
                    &mut self.gl,
                );
            }
        }

        self.gl.draw_end();
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for i in 0..self.gamestate.herbivores.len() {
            let mut kill_herbivore = false;
            if let Some(herbivore) = self.gamestate.herbivores[i].as_mut() {
                if herbivore.energy_level == 0 {
                    kill_herbivore = true;
                } else {
                    herbivore.advance();
                    for u in 0..self.gamestate.plants.len() {
                        herbivore.try_eat(&mut self.gamestate.plants[u]);
                    }
                }
            }
            if kill_herbivore {
                self.gamestate.herbivores[i] = None;
            }
        }
    }

    pub fn new(gl: GlGraphics, gamestate: GameState) -> App {
        App {
            gl: gl,
            gamestate: gamestate,
        }
    }
}
