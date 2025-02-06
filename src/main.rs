mod terminal;
mod fractal;

use std::env;

fn help() {
    println!("cargo run");
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        help();
        return;
    }

    let mut terminal = terminal::Terminal::new();
    let size = terminal.get_size();
    let mut fractal = fractal::Fractal::new(size);

    terminal.initiate_terminal();

    loop {
        if terminal.handle_input(&mut fractal) {
            break;
        }

        terminal.now();
        if terminal.check_time() {
            fractal.update();
            fractal.render(&terminal);
        }
    }

    terminal.clear_terminal();

}
