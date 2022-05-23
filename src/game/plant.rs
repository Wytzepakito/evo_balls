use graphics::types::ColorComponent;

pub struct Plant {
    pub size: f64,
    pub color: [ColorComponent; 4],
    pub x: f64,
    pub y: f64,
    pub alive: bool,
    pub dead_ticks: u32,
    pub energy: u32,
}

impl Plant {
    pub fn new(
        size: f64,
        color: [ColorComponent; 4],
        x: f64,
        y: f64,
        alive: bool,
        dead_ticks: u32,
        energy: u32,
    ) -> Plant {
        Plant {
            size: size,
            color: color,
            x: x,
            y: y,
            alive: alive,
            dead_ticks: dead_ticks,
            energy: energy,
        }
    }
}
