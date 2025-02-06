mod terminal;

use std::env;

fn help() {
    println!("cargo run -- manual : Run the program in manual mode");
    println!("cargo run -- auto : Run the program in automatic mode");
}

fn main() {

    // let args: Vec<String> = env::args().collect();
    // if args.len() != 2 {
    //     help();
    //     return;
    // } else if args[1] == "manual" {
    // } else if args[1] == "auto" {
    // } else {
    //     help();
    //     return;
    // }

    let mut terminal = terminal::Terminal::new();

    terminal.initiate_terminal();

    loop {
        if terminal.handle_input() {
            break;
        }

        terminal.now();
        terminal.check_time();
    }

    terminal.clear_terminal();

}
