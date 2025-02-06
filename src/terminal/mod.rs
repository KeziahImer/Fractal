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

    pub fn check_time(&mut self) {
        if self.elapsed >= self.frame_duration {
            self.last_frame = self.now;
        } else {
            sleep(self.frame_duration - self.elapsed);
        }
    }

    pub fn handle_input(&self) -> bool {
        if event::poll(Duration::from_millis(1)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == KeyCode::Char('d') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn draw(&self, x: f32, y: f32, c: char) {
        let resized_x: u16 = (x + (self.cols / 2) as f32).floor() as u16;
        let resized_y: u16 = (y + (self.rows / 2) as f32).floor() as u16;
        execute!(io::stdout(), crossterm::cursor::MoveTo(resized_x, resized_y), crossterm::style::Print(c)).unwrap();
    }

    pub fn clear(&self) {
        execute!(io::stdout(), Clear(ClearType::All)).unwrap();
    }

    pub fn flush(&self) {
        io::stdout().flush().unwrap();
    }
}
