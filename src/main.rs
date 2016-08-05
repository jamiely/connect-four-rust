extern crate connect_four;
use connect_four::game::*;
use std::io;

fn main() {
    let mut game = Game::new();
    println!("Type \"quit\" or \"q\" to quit");
    loop {
        let mut buffer = String::new();
        println!("What column would you like to play {:?} (0-{})?\n{}",
                 game.current_marker,
                 game.board.columns,
                 game.board);
        io::stdin().read_line(&mut buffer)
            .ok().expect("Failed to read line");

        let line = buffer.trim();

        match line {
            "quit" | "q" => return,
            _ => {
                match line.parse() {
                    Ok(input) => match game.make_move(input) {
                        Some(_) => println!("{}", input),
                        None    => println!("Cannot make that move"),
                    },
                    Err(_) => println!("Invalid selection"),
                }
            },
        }

        if ! game.has_moves() {
            println!("\n{}\nThere are no moves left", game.board);
            break;
        }

    }
}

