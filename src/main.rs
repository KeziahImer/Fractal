mod terminal;
mod fractal;

use std::env;

fn help() {
    println!("cargo run <mandelbrot|aesthetic|depth|stability|x y>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut terminal = terminal::Terminal::new();
    let rows = terminal.get_rows();
    let cols = terminal.get_cols();
    let mut fractal: Box<dyn fractal::FractalTrait>;

    if args.len() == 2 {
        if args[1] == "mandelbrot" {
            fractal = Box::new(fractal::Mandelbrot::new(rows, cols));
        } else if args[1] == "aesthetic" {
            fractal = Box::new(fractal::Julia::new(rows, cols, -0.8, 0.156));
        } else if args[1] == "depth" {
            fractal = Box::new(fractal::Julia::new(rows, cols, 0.285, 0.01));
        } else if args[1] == "stability" {
            fractal = Box::new(fractal::Julia::new(rows, cols, -0.4, 0.6));
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
        fractal = Box::new(fractal::Julia::new(rows, cols, x, y));
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
