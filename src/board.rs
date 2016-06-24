use std::collections::HashMap;

pub type Index = (usize, usize);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Marker {
    Empty,
    X,
    O,
}

pub struct Board {
    rows: usize,
    columns: usize,
    markers: HashMap<Index, Marker>
}

impl Board {
    pub fn indicies_from_rows_and_columns(rows: usize, columns: usize) -> Vec<Index> {
        (0..columns).flat_map(|column: usize| {
            (0..rows).map(move |row: usize| (column, row))
        }).collect::<Vec<Index>>()
    }
    pub fn indicies(&self) -> Vec<Index> {
        Board::indicies_from_rows_and_columns(self.rows, self.columns)
    }

    pub fn new() -> Board {
        let rows = 6;
        let columns = 7;

        let mut markers = HashMap::new();
        for index in Board::indicies_from_rows_and_columns(rows, columns).clone() {
            markers.insert(index, Marker::Empty);
        }

        Board {
            rows: rows,
            columns: columns,
            markers: markers
        }
    }

    pub fn is_empty(&self) -> bool {
        self.markers.values().all(|v| Marker::Empty.eq(v))
    }

    pub fn set_marker(&mut self, index: &Index, marker: Marker) -> () {
        self.markers.insert(index.clone(), marker);
    }

    pub fn get_marker(&self, index: &Index) -> Option<Marker> {
        self.markers.get(&index).map(|r| r.clone())
    }

    pub fn has_moves(&self) -> bool {
        self.markers.values().any(|v| Marker::Empty.eq(v))
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
        let indices = board.indicies();
        assert!(indices.len() == board.rows * board.columns);
    }

    #[test]
    fn it_should_be_empty() {
        let board = Board::new();
        assert!(board.is_empty());
    }

    #[test]
    fn it_should_keep_track_of_markers() {
        let mut board = Board::new();
        let index = (0, 0);
        board.set_marker(&index, Marker::X);
        match board.get_marker(&index) {
            Some(marker) => assert_eq!(marker, Marker::X),
            None         => panic!(),
        }
    }

    #[test]
    fn it_should_know_when_there_are_no_moves_left() {
        let mut board = Board::new();
        {
            for index in board.indicies() {
                board.set_marker(&index, Marker::X);
            }
        }
        assert!(!board.has_moves());
    }
}




