use crate::terminal::Terminal;

const ZOOM: f32 = 0.1;

pub struct Fractal {
    size: u16,
    max_iterations: u32,
    offset_x: Range,
    offset_y: Range,
}

struct Range {
    min: f32,
    max: f32,
}

struct Coordinate {
    x: f32,
    y: f32,
}

impl Fractal {
    pub fn new(size: u16) -> Fractal {
        Fractal {
            size,
            max_iterations: 10000,
            offset_x: Range { min: -2.0, max: 2.0 },
            offset_y: Range { min: -2.0, max: 2.0 },
        }
    }

    fn calculate(&self, terminal: &Terminal) {
        for i in 0..self.size {
            for j in 0..self.size {
                let x = self.offset_x.min + (i as f32) * ((self.offset_x.max - self.offset_x.min) / self.size as f32);
                let y = self.offset_y.min + (j as f32) * ((self.offset_y.max - self.offset_y.min) / self.size as f32);
                let mut z = Coordinate { x: 0.0, y: 0.0 };
                let c = Coordinate { x, y };
                let mut n = 0;
                while z.x * z.x + z.y * z.y <= 2.0 * 2.0 && n < self.max_iterations {
                    let x_new = z.x * z.x - z.y * z.y + c.x;
                    z.y = 2.0 * z.x * z.y + c.y;
                    z.x = x_new;
                    n += 1;
                }
                terminal.draw(i as f32, j as f32, n);
            }
        }
    }

    pub fn zoom_in(&mut self) {
        let zoom = (self.offset_x.max - self.offset_x.min) * ZOOM;
        self.offset_x.min += zoom;
        self.offset_x.max -= zoom;
        self.offset_y.min += zoom;
        self.offset_y.max -= zoom;
    }

    pub fn zoom_out(&mut self) {
        let zoom = (self.offset_x.max - self.offset_x.min) * ZOOM;
        self.offset_x.min -= zoom;
        self.offset_x.max += zoom;
        self.offset_y.min -= zoom;
        self.offset_y.max += zoom;
    }

    pub fn move_up(&mut self) {
        let step = 0.1 * (self.offset_x.max - self.offset_x.min);
        self.offset_y.min -= step;
        self.offset_y.max -= step;
    }

    pub fn move_down(&mut self) {
        let step = 0.1 * (self.offset_x.max - self.offset_x.min);
        self.offset_y.min += step;
        self.offset_y.max += step;
    }

    pub fn move_left(&mut self) {
        let step = 0.1 * (self.offset_x.max - self.offset_x.min);
        self.offset_x.min -= step;
        self.offset_x.max -= step;
    }

    pub fn move_right(&mut self) {
        let step = 0.1 * (self.offset_x.max - self.offset_x.min);
        self.offset_x.min += step;
        self.offset_x.max += step;
    }

    pub fn update(&self) {
    }
    
    pub fn render(&self, terminal: &Terminal) {
        // terminal.clear();
        self.calculate(terminal);
        terminal.flush();
    }
}

