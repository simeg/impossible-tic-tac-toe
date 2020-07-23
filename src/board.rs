use crate::board::CellValue::Empty;

#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub enum CellValue {
    Empty = 0,
    CPU = 1,
    Human = 2,
}

#[derive(Serialize, Debug, Clone)]
pub struct Cell {
    pub row: u8,
    pub column: u8,
    pub value: CellValue,
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        self.value == Empty
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        (self.row, self.column) == (other.row, other.column)
    }
}

pub struct Board {
    cells: Vec<Cell>,
}

impl Board {
    pub fn new() -> Board {
        let mut cells = vec![];
        for row in 0..=2 {
            for column in 0..=2 {
                cells.push(Board::empty_cell(row, column))
            }
        }

        Board { cells }
    }

    pub fn get_cells(&self) -> Vec<Cell> {
        self.cells.clone()
    }

    pub fn is_moves_left(cells: Vec<Cell>) -> bool {
        cells.into_iter().any(|c| c.value == Empty)
    }

    pub fn get_cell(cells: Vec<Cell>, row: u8, column: u8) -> Option<Cell> {
        if let Some(index) = cells
            .iter()
            .position(|c| (c.row, c.column) == (row, column))
        {
            return Some(cells.clone().get(index).unwrap().to_owned());
        }

        None
    }

    pub fn set_all(&mut self, cells: Vec<Cell>) {
        self.cells = cells;
    }

    pub fn set_human(&mut self, row: u8, column: u8) -> Option<Vec<Cell>> {
        self.set(Cell {
            row,
            column,
            value: CellValue::Human,
        })
    }

    pub fn set_cpu(&mut self, row: u8, column: u8) -> Option<Vec<Cell>> {
        self.set(Cell {
            row,
            column,
            value: CellValue::CPU,
        })
    }

    fn set(&mut self, cell: Cell) -> Option<Vec<Cell>> {
        let maybe_set = Board::set_on(self.cells.clone(), cell);
        // Replace if available
        maybe_set.clone().map(|cells| self.set_all(cells));
        maybe_set
    }

    pub fn set_on(mut cells: Vec<Cell>, cell: Cell) -> Option<Vec<Cell>> {
        if let Some(index) = cells.iter().position(|c| c == &cell) {
            let old_cell = cells.get(index).unwrap();
            if old_cell.is_empty() {
                std::mem::replace(&mut cells[index], cell);
                return Some(cells);
            }
        }

        None
    }

    fn empty_cell(row: u8, column: u8) -> Cell {
        Cell {
            row,
            column,
            value: Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate::board::CellValue::{Empty, Human};
    use crate::board::{Board, Cell, CellValue};

    #[test]
    fn test_new_board__creates_cells() {
        let board = Board::new();
        let cells = board.get_cells();

        let actual = cells.len();
        let expected = 9;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_cell__returns_correct_cell() {
        let board = Board::new();
        let cells = board.get_cells();

        let actual = Board::get_cell(cells, 1, 1);
        let expected = Some(Cell {
            row: 1,
            column: 1,
            value: Empty,
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_moves_left__when__moves_left() {
        let board = Board::new();
        let actual = Board::is_moves_left(board.get_cells());
        let expected = true;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_moves_left__when__no_moves_left() {
        let cells = vec![(0, 0), (0, 1), (0, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Human,
            })
            .collect();

        let actual = Board::is_moves_left(cells);
        let expected = false;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_moves_left__when__some_moves_left() {
        let mut cells: Vec<Cell> = vec![(0, 0), (0, 1), (0, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Human,
            })
            .collect();

        // Push a single cell that is playable
        cells.push(Cell {
            row: 1,
            column: 1,
            value: Empty,
        });

        let actual = Board::is_moves_left(cells);
        let expected = true;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_set_all() {
        let mut board = Board::new();

        let before_replace_len = board.get_cells().len();
        assert_eq!(before_replace_len, 9);

        let cells: Vec<Cell> = vec![Cell {
            row: 100,
            column: 100,
            value: CellValue::Human,
        }];
        board.set_all(cells);

        let actual = board.get_cells().len();
        let expected = 1;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_set() {
        let mut board = Board::new();
        let before_set = Board::get_cell(board.get_cells(), 1, 1);
        let before_set_cell = Some(Cell {
            row: 1,
            column: 1,
            value: CellValue::Empty,
        });
        assert_eq!(before_set, before_set_cell);

        let human_cell = Cell {
            row: 1,
            column: 1,
            value: CellValue::Human,
        };
        let set_result = board.set(human_cell);
        assert!(set_result.is_some());

        assert_eq!(board.get_cells().len(), 9);

        let actual = Board::get_cell(board.get_cells(), 1, 1);
        let expected = Some(Cell {
            row: 1,
            column: 1,
            value: CellValue::Human,
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_set__when__not_empty() {
        let mut board = Board::new();
        let cell = Cell {
            row: 1,
            column: 1,
            value: CellValue::Human,
        };
        let set_result = board.set(cell);
        assert!(set_result.is_some());

        let cell_after_set = Board::get_cell(board.get_cells(), 1, 1);
        let cell_after_set_expected = Some(Cell {
            row: 1,
            column: 1,
            value: CellValue::Human,
        });
        assert_eq!(cell_after_set, cell_after_set_expected);

        let cell2 = Cell {
            row: 1,
            column: 1,
            value: CellValue::Human,
        };
        let actual = board.set(cell2);
        let expected = None;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_set__when__out_of_range() {
        let mut board = Board::new();
        let cell = Cell {
            row: 100,
            column: 100,
            value: CellValue::Human,
        };

        let actual = board.set(cell);
        let expected = None;

        assert_eq!(actual, expected);
    }
}
