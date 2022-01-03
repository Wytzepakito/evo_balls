use game::game_state::{Circle, Direction, GameState};
use graphics::{clear, color::RED, ellipse, Transformed};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use rand::Rng;

mod game;

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

        for circle in &self.gamestate.circles {
            let c = context.trans(circle.x - circle.size, circle.y - circle.size);
            ellipse(
                circle.color,
                [circle.size, circle.size, circle.size, circle.size],
                c.transform,
                &mut self.gl,
            );
        }

        self.gl.draw_end();
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for i in 0..self.gamestate.circles.len() {
            self.gamestate.circles[i].advance();
        }
    }

    pub fn new(gl: GlGraphics, gamestate: GameState) -> App {
        App {
            gl: gl,
            gamestate: gamestate,
        }
    }
}

pub fn create_gamestate(n: usize) -> GameState {
    let mut circles: Vec<Circle> = Vec::new();
    for i in 0..n {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..800.0) as f64;
        let y = rng.gen_range(0.0..800.0) as f64;
        let direction = match rng.gen_range(0..7) {
            0 => Direction::North,
            1 => Direction::NorthWest,
            2 => Direction::West,
            3 => Direction::SouthWest,
            4 => Direction::South,
            5 => Direction::SouthEast,
            6 => Direction::East,
            7 => Direction::NorthEast,
            _ => unimplemented!(),
        };

        let mut circ = Circle::new(50.0, RED, x, y, direction);

        circles.push(circ);
    }

    GameState { circles: circles }
}
