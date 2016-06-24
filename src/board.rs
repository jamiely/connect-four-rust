use std::collections::HashMap;

pub struct Board {
    rows: usize,
    columns: usize,
    indicies: Vec<(usize, usize)>,
    markers: HashMap<(usize, usize), usize>
}

impl Board {
    pub fn new() -> Board {
        let rows = 6;
        let columns = 7;

        let indicies = (0..columns).flat_map(|column: usize| {
            (0..rows).map(move |row: usize| (column, row))
        }).collect::<Vec<(usize, usize)>>();

        let mut markers = HashMap::new();
        for index in indicies.clone() {
            markers.insert(index, 0 as usize);
        }

        Board {
            rows: rows,
            columns: columns,
            indicies: indicies,
            markers: markers
        }
    }

    pub fn is_empty(&self) -> bool {
        let zero: usize = 0;
        self.markers.values().all(|v| zero.eq(v))
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
        let indices = board.indicies;
        assert!(indices.len() == board.rows * board.columns);
    }

    #[test]
    fn it_should_be_empty() {
        let board = Board::new();
        assert!(board.is_empty());
    }
}




