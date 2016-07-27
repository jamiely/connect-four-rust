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
    use board::*;

    #[test]
    fn starts_with_x() {
        let game = Game::new();
        assert!(game.current_marker == Marker::X)
    }

    fn assert_index(opt_index: Option<Index>, index: Index) -> () {
        match opt_index {
            Some(i) if i == index => assert!(true),
            Some(_) => assert!(false),
            None => assert!(false),
        }
    }

    fn assert_none(opt_index: Option<Index>) -> () {
        match opt_index {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn current_marker_changes_when_move_is_made() {
        let mut game = Game::new();
        assert!(game.current_marker == Marker::X);
        game.make_move(0);
        assert!(game.current_marker == Marker::O);
    }

    #[test]
    fn moves_increment_indices() {
        let mut game = Game::new();
        assert_index(game.make_move(0), (0, 0));
        assert_index(game.make_move(0), (0, 1));
        assert_index(game.make_move(0), (0, 2));
        assert_index(game.make_move(0), (0, 3));
        assert_index(game.make_move(0), (0, 4));
        assert_index(game.make_move(0), (0, 5));
        assert_none(game.make_move(0));
    }
}

