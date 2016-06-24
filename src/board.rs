use std::collections::HashMap;

type Index = (usize, usize);

#[derive(Copy, Clone, Debug, PartialEq)]
enum Marker {
    Empty,
    X,
    O,
}

pub struct Board {
    rows: usize,
    columns: usize,
    indicies: Vec<Index>,
    markers: HashMap<Index, Marker>
}

impl Board {
    pub fn new() -> Board {
        let rows = 6;
        let columns = 7;

        let indicies = (0..columns).flat_map(|column: usize| {
            (0..rows).map(move |row: usize| (column, row))
        }).collect::<Vec<Index>>();

        let mut markers = HashMap::new();
        for index in indicies.clone() {
            markers.insert(index, Marker::Empty);
        }

        Board {
            rows: rows,
            columns: columns,
            indicies: indicies,
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
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Marker;

    #[test]
    fn should_have_7_columns_and_6_rows() {
        let board = Board::new();
        assert!(board.rows == 6);
        assert!(board.columns == 7)
    }

    #[test]
    fn it_should_have_rows_times_columns_indicies() {
        let board = Board::new();
        let indices = board.indicies;
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
}




