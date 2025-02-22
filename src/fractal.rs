use crate::terminal::Terminal;

const RATIO: f32 = 0.1;

struct Range {
    min: f32,
    max: f32,
}

struct Coordinate {
    x: f32,
    y: f32,
}

pub trait FractalTrait {
    fn render(&self, terminal: &Terminal);
    fn calculate(&self, terminal: &Terminal);
    fn zoom_in(&mut self);
    fn zoom_out(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
}

pub struct Fractal {
    rows: u16,
    cols: u16,
    offset_x: Range,
    offset_y: Range,
}

pub struct Mandelbrot {
    pub fractal: Fractal,
    max_iterations: u32,
}

impl Mandelbrot {
    pub fn new(rows: u16, cols: u16) -> Mandelbrot {
        Mandelbrot {
            fractal: Fractal {
                rows,
                cols,
                offset_x: Range { min: -2.0, max: 2.0 },
                offset_y: Range { min: -2.0, max: 2.0 },
            },
            max_iterations: 10000,
        }
    }
}

impl FractalTrait for Mandelbrot {
    fn render(&self, terminal: &Terminal) {
        self.calculate(terminal);
        terminal.flush();
    }

    fn calculate(&self, terminal: &Terminal) {
        for i in 0..self.fractal.rows {
            for j in 0..self.fractal.cols {
                let x = self.fractal.offset_x.min + (j as f32) * ((self.fractal.offset_x.max - self.fractal.offset_x.min) / self.fractal.cols as f32);
                let y = self.fractal.offset_y.min + (i as f32) * ((self.fractal.offset_y.max - self.fractal.offset_y.min) / self.fractal.rows as f32);
                let mut z = Coordinate { x: 0.0, y: 0.0 };
                let c = Coordinate { x, y };
                let mut n = 0;
                while z.x * z.x + z.y * z.y <= 2.0 * 2.0 && n < self.max_iterations {
                    let x_new = z.x * z.x - z.y * z.y + c.x;
                    z.y = 2.0 * z.x * z.y + c.y;
                    z.x = x_new;
                    n += 1;
                }
                terminal.draw(i, j, n, self.max_iterations);
            }
        }
    }

    fn zoom_in(&mut self) {
        let zoom = (self.fractal.offset_x.max - self.fractal.offset_x.min) * RATIO;
        self.fractal.offset_x.min += zoom;
        self.fractal.offset_x.max -= zoom;
        self.fractal.offset_y.min += zoom;
        self.fractal.offset_y.max -= zoom;
    }

    fn zoom_out(&mut self) {
        let zoom = (self.fractal.offset_x.max - self.fractal.offset_x.min) * RATIO;
        self.fractal.offset_x.min -= zoom;
        self.fractal.offset_x.max += zoom;
        self.fractal.offset_y.min -= zoom;
        self.fractal.offset_y.max += zoom;
    }

    fn move_up(&mut self) {
        let step = RATIO * (self.fractal.offset_x.max - self.fractal.offset_x.min);
        self.fractal.offset_y.min -= step;
        self.fractal.offset_y.max -= step;
    }

    fn move_down(&mut self) {
        let step = RATIO * (self.fractal.offset_x.max - self.fractal.offset_x.min);
        self.fractal.offset_y.min += step;
        self.fractal.offset_y.max += step;
    }

    fn move_left(&mut self) {
        let step = RATIO * (self.fractal.offset_x.max - self.fractal.offset_x.min);
        self.fractal.offset_x.min -= step;
        self.fractal.offset_x.max -= step;
    }

    fn move_right(&mut self) {
        let step = RATIO * (self.fractal.offset_x.max - self.fractal.offset_x.min);
        self.fractal.offset_x.min += step;
        self.fractal.offset_x.max += step;
    }
}

pub struct Julia {
    fractal: Fractal,
    c: Coordinate,
    max_iterations: u32,
}

impl Julia {
    pub fn new(rows: u16, cols: u16, x: f32, y: f32) -> Julia {
        Julia {
            fractal: Fractal {
                rows,
                cols,
                offset_x: Range { min: -2.0, max: 2.0 },
                offset_y: Range { min: -2.0, max: 2.0 },
            },
            c: Coordinate { x, y },
            max_iterations: 5000,
        }
    }
}

impl FractalTrait for Julia {
    fn render(&self, terminal: &Terminal) {
        self.calculate(terminal);
        terminal.flush();
    }

    fn calculate(&self, terminal: &Terminal) {
        for i in 0..self.fractal.rows {
            for j in 0..self.fractal.cols {
                let x = self.fractal.offset_x.min + (j as f32) * ((self.fractal.offset_x.max - self.fractal.offset_x.min) / self.fractal.cols as f32);
                let y = self.fractal.offset_y.min + (i as f32) * ((self.fractal.offset_y.max - self.fractal.offset_y.min) / self.fractal.rows as f32);
                let mut z = Coordinate { x, y };
                let mut n = 0;
                while z.x * z.x + z.y * z.y <= 2.0 * 2.0 && n < self.max_iterations {
                    let x_new = z.x * z.x - z.y * z.y + self.c.x;
                    z.y = 2.0 * z.x * z.y + self.c.y;
                    z.x = x_new;
                    n += 1;
                }
                terminal.draw(i, j, n, self.max_iterations);
            }
        }
    }

    fn zoom_in(&mut self) {
        let zoom = (self.fractal.offset_x.max - self.fractal.offset_x.min) * RATIO;
        self.fractal.offset_x.min += zoom;
        self.fractal.offset_x.max -= zoom;
        self.fractal.offset_y.min += zoom;
        self.fractal.offset_y.max -= zoom;
    }

    fn zoom_out(&mut self) {
        let zoom = (self.fractal.offset_x.max - self.fractal.offset_x.min) * RATIO;
        self.fractal.offset_x.min -= zoom;
        self.fractal.offset_x.max += zoom;
        self.fractal.offset_y.min -= zoom;
        self.fractal.offset_y.max += zoom;
    }

    fn move_up(&mut self) {
        let step = RATIO * (self.fractal.offset_x.max - self.fractal.offset_x.min);
        self.fractal.offset_y.min -= step;
        self.fractal.offset_y.max -= step;
    }

    fn move_down(&mut self) {
        let step = RATIO * (self.fractal.offset_x.max - self.fractal.offset_x.min);
        self.fractal.offset_y.min += step;
        self.fractal.offset_y.max += step;
    }

    fn move_left(&mut self) {
        let step = RATIO * (self.fractal.offset_x.max - self.fractal.offset_x.min);
        self.fractal.offset_x.min -= step;
        self.fractal.offset_x.max -= step;
    }

    fn move_right(&mut self) {
        let step = RATIO * (self.fractal.offset_x.max - self.fractal.offset_x.min);
        self.fractal.offset_x.min += step;
        self.fractal.offset_x.max += step;
    }
}

