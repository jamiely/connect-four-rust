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

        let current_marker = game.current_marker;
        let line = buffer.trim();

        match line {
            "quit" | "q" => return,
            _ => match line.parse().ok().and_then(|input| game.make_move(input)) {
                Some(index) if game.is_win(index) => {
                    println!("{}\n{} has won!", game.board, current_marker);
                    break;
                },
                Some(_) => {},
                None    => println!("Cannot make that move"),
            }
        }

        if ! game.has_moves() {
            println!("\n{}\nThere are no moves left", game.board);
            break;
        }

    }
}

