mod terminal;
mod fractal;

use std::env;

fn help() {
    println!("cargo run <spiral|cosmicarc|fractalburst|celestialspiral|heart|mandelbulb|cycloneeye|goldenspiral|flame>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut terminal = terminal::Terminal::new();
    let rows = terminal.get_rows();
    let cols = terminal.get_cols();
    let mut fractal: Box<dyn fractal::FractalTrait>;

    if args.len() == 2 {
        if args[1] == "mandelbrot" {
            fractal = Box::new(fractal::Mandelbrot::new(rows, cols, 10000));
        } else if args[1] == "spiral" {
            fractal = Box::new(fractal::Julia::new(rows, cols, -0.70176, 0.3842, 500));
        } else if args[1] == "cosmicarc" {
            fractal = Box::new(fractal::Julia::new(rows, cols, 0.285, 0.01, 500));
        } else if args[1] == "fractalburst" {
            fractal = Box::new(fractal::Julia::new(rows, cols, -0.8, 0.156, 500));
        } else if args[1] == "celestialspiral" {
            fractal = Box::new(fractal::Julia::new(rows, cols, 0.8, 0.156, 500));
        } else if args[1] == "heart" {
            fractal = Box::new(fractal::Julia::new(rows, cols, -0.4, 0.6, 500));
        } else if args[1] == "mandelbulb" {
            fractal = Box::new(fractal::Julia::new(rows, cols, -1.0, 0.0, 500));
        } else if args[1] == "cycloneeye" {
            fractal = Box::new(fractal::Julia::new(rows, cols, 0.1, 0.65, 500));
        } else if args[1] == "goldenspiral" {
            fractal = Box::new(fractal::Julia::new(rows, cols, 0.355, 0.355, 500));
        } else if args[1] == "flame" {
            fractal = Box::new(fractal::Julia::new(rows, cols, 0.3, 0.4, 500));
        } else {
            help();
            return;
        }
    } else if args.len() == 3 {
        let x: f32 = args[1].parse().unwrap_or_else(|_| {
            help();
            std::process::exit(1);
        });
        let y: f32 = args[2].parse().unwrap_or_else(|_| {
            help();
            std::process::exit(1);
        });
        fractal = Box::new(fractal::Julia::new(rows, cols, x, y, 500));
    } else {
        help();
        return;
    }

    terminal.initiate_terminal();

    loop {
        if terminal.handle_input(&mut *fractal) {
            break;
        }

        terminal.now();
        if terminal.check_time() {
            fractal.render(&terminal);
        }
    }

    terminal.clear_terminal();
}
