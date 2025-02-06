use crate::terminal::Terminal;

const ZOOM_IN: f32 = 0.9;
const ZOOM_OUT: f32 = 1.1;

pub struct Fractal {
    size: u16,
    max_iterations: u32,
    offset: Complex,
}

struct Complex {
    x: f32,
    y: f32,
}

impl Fractal {
    pub fn new(size: u16) -> Fractal {
        Fractal {
            size,
            max_iterations: 100,
            offset: Complex { x: 2.0, y: 2.0 },
        }
    }

    fn calculate(&self, terminal: &Terminal) {
        for i in 0..self.size {
            for j in 0..self.size {
                let x = (i as f32 - self.size as f32 / 2.0) / self.size as f32 * self.offset.x;
                let y = (j as f32 - self.size as f32 / 2.0) / self.size as f32 * self.offset.y;
                let mut z = Complex { x, y };
                let c = Complex { x: 0.0, y: 0.0 };
                let mut n = 0;
                while z.x * z.x + z.y * z.y <= 2.0 * 2.0 && n < self.max_iterations {
                    let x_new = z.x * z.x - z.y * z.y + c.x;
                    z.y = 2.0 * z.x * z.y + c.y;
                    z.x = x_new;
                    n += 1;
                }
                let brightness = (n as f32 / self.max_iterations as f32 * 255.0) as u8;
                terminal.draw(z.x, z.y, '*');
            }
        }
    }

    pub fn zoom_in(&mut self) {
        self.offset.x *= ZOOM_IN;
        self.offset.y *= ZOOM_IN;
    }

    pub fn zoom_out(&mut self) {
        self.offset.x *= ZOOM_OUT;
        self.offset.y *= ZOOM_OUT;
    }

    pub fn move_up(&mut self) {
        self.offset.y -= 0.1;
    }

    pub fn move_down(&mut self) {
        self.offset.y += 0.1;
    }

    pub fn move_left(&mut self) {
        self.offset.x -= 0.1;
    }

    pub fn move_right(&mut self) {
        self.offset.x += 0.1;
    }

    pub fn update(&self) {

    }

    pub fn render(&self, terminal: &Terminal) {
        terminal.clear();
        self.calculate(terminal);
        terminal.flush();
    }
}

