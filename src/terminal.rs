use crate::fractal::Fractal;

use crossterm::{execute, cursor::{Hide, Show}, terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType}};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::thread::sleep;

const FPS: u64 = 5;

pub struct Terminal {
    size: u16,
    cols: u16,
    rows: u16,
    frame_duration: Duration,
    last_frame: Instant,
    now: Instant,
    elapsed: Duration,
}

impl Terminal {
    pub fn new() -> Terminal {
        let (cols, rows) = size().unwrap();
        Terminal {
            cols,
            rows,
            size: if cols < rows { cols } else { rows },
            frame_duration: Duration::from_secs_f64(1.0 / FPS as f64),
            last_frame: Instant::now(),
            now: Instant::now(),
            elapsed: Duration::new(0, 0),
        }
    }

    pub fn initiate_terminal(&self) {
        enable_raw_mode().unwrap();
        execute!(io::stdout(), Hide, Clear(ClearType::All)).unwrap();
    }
    
    pub fn clear_terminal(&self) {
        execute!(io::stdout(), Show, Clear(ClearType::All)).unwrap();
        disable_raw_mode().unwrap();
    }

    pub fn now(&mut self) {
        self.now = Instant::now();
        self.elapsed = self.now.duration_since(self.last_frame);
    }

    pub fn check_time(&mut self) -> bool {
        if self.elapsed >= self.frame_duration {
            self.last_frame = self.now;
            return true;
        } else {
            sleep(self.frame_duration - self.elapsed);
        }
        return false;
    }

    pub fn get_size(&self) -> u16 {
        self.size
    }

    pub fn handle_input(&self, fractal: &mut Fractal) -> bool {
        if event::poll(Duration::from_millis(1)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == KeyCode::Char('d') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    return true;
                }
                if key_event.code == KeyCode::Char('e') {
                    fractal.zoom_in();
                }
                if key_event.code == KeyCode::Char('r') {
                    fractal.zoom_out();
                }
                if key_event.code == KeyCode::Left {
                    fractal.move_left();
                }
                if key_event.code == KeyCode::Right {
                    fractal.move_right();
                }
                if key_event.code == KeyCode::Up {
                    fractal.move_up();
                }
                if key_event.code == KeyCode::Down {
                    fractal.move_down();
                }
            }
        }
        return false;
    }

    pub fn draw(&self, x: f32, y: f32, n: u32) {
        let resized_x: u16 = (x + (self.cols / 2) as f32).floor() as u16;
        let resized_y: u16 = (y + (self.rows / 2) as f32).floor() as u16;
        let c;
        if n < 100 {
            c = ' ';
        } else if n < 1000 {
            c = 'a';
        } else if n < 2000 {
            c = 'b';
        } else if n < 3000 {
            c = 'c';
        } else if n < 4000 {
            c = 'd';
        } else if n < 5000 {
            c = 'e';
        } else if n < 6000 {
            c = 'f';
        } else if n < 7000 {
            c = 'g';
        } else if n < 8000 {
            c = 'h';
        } else if n < 9000 {
            c = 'i';
        } else {
            c = 'j';
        }
        execute!(io::stdout(), crossterm::cursor::MoveTo(resized_x, resized_y), crossterm::style::Print(c)).unwrap();
    }

    pub fn clear(&self) {
        execute!(io::stdout(), Clear(ClearType::All)).unwrap();
        execute!(io::stdout(), Clear(ClearType::Purge)).unwrap();
    }

    pub fn flush(&self) {
        io::stdout().flush().unwrap();
    }
}
