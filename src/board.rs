use std::collections::HashMap;
use std::fmt;

pub type Index = (usize, usize);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Marker {
    Empty,
    X,
    O,
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match *self {
            Marker::X => "X",
            Marker::O => "Y",
            _ => "_"
        };

        write!(f, "{}", result)
    }
}

pub struct Board {
    pub rows: usize,
    pub columns: usize,
    markers: HashMap<Index, Marker>
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result: String = (0..self.rows).rev().map(|row: usize| {
            (0..self.columns).map(move |column: usize| {
                match self.get_marker(&(column, row)) {
                    Some(marker) => format!("{}", marker),
                    None => "_".to_owned(),
                }.to_owned()
            }).collect::<Vec<String>>().join("").to_owned()
        }).collect::<Vec<String>>().join("\n");
        write!(f, "{}", result)
    }
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

    pub fn index_is_valid(&self, index: &Index) -> bool {
        let (col, row) = *index;
        row < self.rows && col < self.columns
    }

    // Returns true if the passed marker is at the passed index
    pub fn marker_is_at(&self, marker: Marker, index: &Index) -> bool {
        if self.index_is_valid(index) {
            match self.get_marker(index) {
                Some(m) => m == marker,
                None => false,
            }
        }
        else {
            false
        }
    }

    pub fn drop_into_column(&mut self, column: usize, marker: Marker) -> Option<Index> {
        match self.first_empty_row_in_column(column) {
            Some(index) => {
                self.set_marker(&index, marker);
                Some(index)
            }
            None => None
        }
    }

    fn first_empty_row_in_column(&self, column: usize) -> Option<Index> {
        (0..self.rows).map(|row| (column, row)).find(|index| {
            match self.get_marker(&index) {
                Some(m) => m == Marker::Empty,
                None => false
            }
        })
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
        let indicies = board.indicies();
        assert!(indicies.len() == board.rows * board.columns);
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

    #[test]
    fn it_checks_for_valid_indicies() {
        let board = Board::new();
        assert!(board.index_is_valid(&(1,1)));
        assert!(!board.index_is_valid(&(1,10)));
        assert!(!board.index_is_valid(&(10,1)));
    }

    #[test]
    fn can_check_for_marker_at() {
        let mut board = Board::new();

        assert!(!board.marker_is_at(Marker::X, &(1,1)));
        board.set_marker(&(1,1), Marker::X);
        assert!(board.marker_is_at(Marker::X, &(1,1)));
    }

    #[test]
    fn it_finds_the_first_empty_row_in_a_column() {
        let mut board = Board::new();
        match board.first_empty_row_in_column(0) {
            Some(index) => assert_eq!(index, (0,0)),
            None => panic!()
        };

        board.set_marker(&(0,0), Marker::X);

        match board.first_empty_row_in_column(0) {
            Some(index) => assert_eq!(index, (0,1)),
            None => panic!()
        };

        board.set_marker(&(0,1), Marker::X);

        match board.first_empty_row_in_column(0) {
            Some(index) => assert_eq!(index, (0,2)),
            None => panic!()
        };
    }

    #[test]
    fn it_allows_dropping_into_a_column() {
        let mut board = Board::new();
        assert!(board.drop_into_column(0, Marker::X).is_some());
        assert!(board.drop_into_column(0, Marker::X).is_some());
        assert!(board.drop_into_column(0, Marker::X).is_some());
        assert!(board.drop_into_column(0, Marker::X).is_some());
        assert!(board.drop_into_column(0, Marker::X).is_some());
        assert!(board.drop_into_column(0, Marker::X).is_some());
        match board.first_empty_row_in_column(0) {
            Some(index) => panic!(),
            None => assert!(true)
        };
    }
}




