pub struct Board {
    rows: i32,
    columns: i32
}

impl Board {
    pub fn new() -> Board {
        Board {
            rows: 6,
            columns: 7
        }
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
}




