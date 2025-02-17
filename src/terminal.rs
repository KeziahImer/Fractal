use crate::fractal::FractalTrait;

use crossterm::{execute, cursor::{Hide, Show}, terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType}};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::thread::sleep;

const FPS: u64 = 60;

pub struct Terminal {
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

    pub fn get_rows(&self) -> u16 {
        self.rows
    }

    pub fn get_cols(&self) -> u16 {
        self.cols
    }

    pub fn handle_input(&self, fractal: &mut dyn FractalTrait) -> bool {
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

    pub fn draw(&self, x: u16, y: u16, n: u32, max_iterations: u32) {
        let c;
        if n < 10 {
            c = ' ';
        } else if n < max_iterations / 200 { // 200
            c = '.';
        } else if n < max_iterations / 100 { // 100
            c = ':';
        } else if n < max_iterations / 20 { // 20
            c = '-';
        } else if n < max_iterations / 10 { // 10
            c = '=';
        } else if n < max_iterations / 5 { // 5
            c = '+';
        } else if n < max_iterations / 2 - 2 * max_iterations / 5 { // 2 - 2/5
            c = '*';
        } else if n < max_iterations / 2 - max_iterations / 5 { // 2 - 1/5
            c = '%';
        } else if n < max_iterations / 2 { // 2
            c = '#';
        } else {
            c = '@';
        }
        execute!(io::stdout(), crossterm::cursor::MoveTo(y, x), crossterm::style::Print(c)).unwrap();
    }

    // pub fn clear(&self) {
    //     execute!(io::stdout(), Clear(ClearType::All)).unwrap();
    // }

    pub fn flush(&self) {
        io::stdout().flush().unwrap();
    }
}
