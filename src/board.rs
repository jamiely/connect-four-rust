pub struct Board {
    rows: usize,
    columns: usize
}

impl Board {
    pub fn new() -> Board {
        Board {
            rows: 6,
            columns: 7
        }
    }
    pub fn indices(&self) -> Vec<(usize, usize)> {
        (0..self.columns).flat_map(|column: usize| {
            (0..self.rows).map(move |row: usize| (column, row))
        }).collect::<Vec<(usize, usize)>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_have_7_columns_and_6_rows() {
        let board = Board::new();
        assert!(board.rows == 6);
        assert!(board.columns == 7)
    }

    #[test]
    fn it_should_have_rows_times_columns_indicies() {
        let board = Board::new();
        let indices = board.indices();
        assert!(indices.len() == board.rows * board.columns);
    }
}




