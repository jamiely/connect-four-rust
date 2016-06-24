use board::Board;

pub struct Game {
    board: Board
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new()
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

}
