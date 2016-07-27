use board::*;

pub struct Game {
    board: Board,
    current_marker: Marker
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new(),
            current_marker: Marker::X
        }
    }

    fn toggle_marker(&mut self) -> Marker {
        self.current_marker = if self.current_marker == Marker::X {
            Marker::O
        } else {
            Marker::X
        };

        self.current_marker
    }

    pub fn make_move(&mut self, column: usize) -> Option<Index> {
        let result = self.board.drop_into_column(column, self.current_marker);
        if result.is_some() {
            self.toggle_marker();
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

}
