use board::*;

pub struct Game {
    pub board: Board,
    pub current_marker: Marker
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_marker: Marker::X
        }
    }

    fn get_marker(&self, index: &Index) -> Option<Marker> {
        self.board.get_marker(index)
    }

    pub fn has_moves(&self) -> bool {
        self.board.indicies().iter().any(|index: &Index| {
            let opt_marker = self.get_marker(index);
            match opt_marker {
                Some(marker) => marker == Marker::Empty,
                None => false
            }
        })
    }

    fn directions(&self) -> Vec<(i8, i8)> {
        vec!((0, 1), (1, 0), (0, -1), (-1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1))
    }

    /*
     * Determine if someone has won at the passed index
     */
    pub fn is_win(&self, index: Index) -> bool {
        match self.get_marker(&index) {
            Some(Marker::Empty) => false,
            Some(marker) => self.directions().iter().any(|dir| {
                                self.is_win_in_dir(marker, index, dir, 1)
                            }),
            None => false
        }
    }

    fn is_win_in_dir(&self, marker: Marker, index: Index, direction: &(i8, i8), count: i8) -> bool {
        if count >= 4 { return true; }
        if marker == Marker::Empty { return false; }

        self.index_in_dir(index, direction).and_then(|next_index| {
            self.get_marker(&next_index).map(|m| (next_index, m))
        }).map_or(false, |pair| {
            let (next_index, next_marker) = pair;
            marker == next_marker &&
                self.is_win_in_dir(marker, next_index, direction, count + 1)
        })
    }

    fn index_in_dir(&self, index: Index, direction: &(i8, i8)) -> Option<Index> {
        let ((col, row), (dc, dr)) = (index, *direction);
        let (nc, nr) = (col as i8 + dc, row as i8 + dr);
        // TODO: fix ugly casts here
        if 0 <= nc && nc < self.board.columns as i8 && 0 <= nr && nr < self.board.rows as i8 {
            Some((nc as usize, nr as usize))
        }
        else {
            None
        }
    }

    pub fn toggle_marker(&mut self) -> Marker {
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

    #[test]
    fn has_moves() {
        let mut game = Game::new();
        assert!(game.has_moves());
        game.board.indicies().iter().fold((), |_, index: &Index| {
            let (col, _) = index.to_owned();
            assert!(game.has_moves());
            game.make_move(col);
        });
        assert!(!game.has_moves());
    }

    #[test]
    fn is_win_in_dir() {
        let mut game = Game::new();

        fn assert_not_win(g: &Game, move_result: Option<Index>) {
            assert!(!g.is_win(move_result.expect("problem")));
        }

        let mut result = game.make_move(0);
        assert!(!game.is_win(result.expect("problem")));
        result = game.make_move(0);
        assert!(!game.is_win(result.expect("problem")));
        result = game.make_move(1);
        assert!(!game.is_win(result.expect("problem")));
        result = game.make_move(1);
        assert!(!game.is_win(result.expect("problem")));
        result = game.make_move(2);
        assert!(!game.is_win(result.expect("problem")));
        result = game.make_move(2);
        assert!(!game.is_win(result.expect("problem")));
        result = game.make_move(3);
        assert!(game.is_win(result.expect("problem")));
    }

    #[test]
    fn vertical_wins() {
        let mut game = Game::new();
        game.make_move(0);
        game.make_move(1);
        game.make_move(0);
        game.make_move(1);
        game.make_move(0);
        let mut result = game.make_move(1);
        assert!(!game.is_win(result.expect("problem")));
        result = game.make_move(0);
        assert!(game.is_win(result.expect("problem")));
    }

    #[test]
    fn diagonal_win() {
        let mut game = Game::new();
        game.make_move(0);
        game.make_move(1);
        game.make_move(1);
        game.make_move(2);
        game.make_move(0);
        game.make_move(2);
        game.make_move(2);
        game.make_move(3);
        game.make_move(3);
        let mut result = game.make_move(3);
        assert!(!game.is_win(result.expect("problem")));
        result = game.make_move(3);
        assert!(game.is_win(result.expect("problem")));
    }
}

