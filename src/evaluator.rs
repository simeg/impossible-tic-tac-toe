use crate::board::CellValue::{Empty, Human, CPU};
use crate::board::{Board, Cell, CellValue};

pub struct Evaluator {}

impl Evaluator {
    pub fn find_best_move(&mut self, cells: Vec<Cell>) -> Cell {
        let is_middle_cell_available = cells
            .clone()
            .into_iter()
            .any(|c| (c.row, c.column, c.value) == (1, 1, Empty));

        if is_middle_cell_available {
            return Cell {
                row: 1,
                column: 1,
                value: CPU,
            };
        }

        if self.is_special_case(cells.clone()) {
            return Cell {
                row: 0,
                column: 2,
                value: CPU,
            };
        }

        // Return move with lowest score
        cells
            .clone()
            .into_iter()
            .filter(Cell::is_empty)
            .map(|c| {
                let cells = Board::set_on(
                    cells.clone(),
                    Cell {
                        row: c.row,
                        column: c.column,
                        value: CellValue::Human,
                    },
                )
                .unwrap();

                let score = self.minimax(cells.clone(), 0);
                (c, score)
            })
            .min_by(|(_c, score), (_c2, score2)| score2.cmp(score))
            .map(|(mut c, _score)| {
                c.value = CPU;
                c
            })
            .unwrap()
    }

    fn minimax(&mut self, cells: Vec<Cell>, depth: u8) -> i8 {
        let score = self.evaluate(cells.clone());

        // If human/CPU has won the game
        // return the evaluated score
        if score == 10 || score == -10 {
            return score;
        }

        // If there are no more moves and
        // no winner then it is a tie
        if !Board::is_moves_left(cells.clone()) {
            return 0;
        }

        // Return lowest score
        cells
            .clone()
            .into_iter()
            .filter(Cell::is_empty)
            .map(|c| {
                let cells = Board::set_on(
                    cells.clone(),
                    Cell {
                        row: c.row,
                        column: c.column,
                        value: CellValue::CPU,
                    },
                )
                .unwrap();

                self.minimax(cells.clone(), (depth + 1) as u8)
            })
            .min_by(|score, score2| score2.cmp(score))
            .unwrap()
    }

    fn evaluate(&self, cells: Vec<Cell>) -> i8 {
        // Check for wins in rows
        for row in 0..=2 {
            let cells_from_same_row = cells
                .clone()
                .into_iter()
                .filter(|c| c.row == row)
                .collect::<Vec<Cell>>();

            let is_complete_row = cells_from_same_row.len() == 3;
            if !is_complete_row {
                continue;
            }

            let human_did_win = cells_from_same_row
                .clone()
                .into_iter()
                .all(|c| c.value == Human);
            if human_did_win {
                // TODO: Replace magic integers with type
                return 10;
            }

            let cpu_did_win = cells_from_same_row
                .clone()
                .into_iter()
                .all(|c| c.value == CPU);
            if cpu_did_win {
                return -10;
            }
        }

        // Check for wins in columns
        for column in 0..=2 {
            let cells_from_same_column = cells
                .clone()
                .into_iter()
                .filter(|c| c.column == column)
                .collect::<Vec<Cell>>();

            let is_complete_column = cells_from_same_column.len() == 3;
            if !is_complete_column {
                continue;
            }

            let human_did_win = cells_from_same_column
                .clone()
                .into_iter()
                .all(|c| c.value == Human);
            if human_did_win {
                return 10;
            }

            let cpu_did_win = cells_from_same_column
                .clone()
                .into_iter()
                .all(|c| c.value == CPU);
            if cpu_did_win {
                return -10;
            }
        }

        // Check for wins in diagonals
        let first_diagonal: Vec<(u8, u8)> =
            vec![(0 as u8, 0 as u8), (1 as u8, 1 as u8), (2 as u8, 2 as u8)];
        let second_diagonal: Vec<(u8, u8)> =
            vec![(0 as u8, 2 as u8), (1 as u8, 1 as u8), (2 as u8, 0 as u8)];
        let diagonals = vec![first_diagonal, second_diagonal];

        let human_did_win = diagonals
            .clone()
            .into_iter()
            .map(|diagonals| self.eval_diagonal_win(cells.clone(), diagonals, Human))
            .any(|res| res);

        if human_did_win {
            return 10;
        }

        let cpu_did_win = diagonals
            .into_iter()
            .map(|diagonals| self.eval_diagonal_win(cells.clone(), diagonals, CPU))
            .any(|res| res);

        if cpu_did_win {
            return -10;
        }

        // No one won
        0
    }

    fn eval_diagonal_win(
        &self,
        cells: Vec<Cell>,
        diagonal: Vec<(u8, u8)>,
        cell_value: CellValue,
    ) -> bool {
        diagonal.len() == 3
            && diagonal
                .into_iter()
                .map(|(row, column)| Board::get_cell(cells.clone(), row, column))
                .map(Option::unwrap)
                .all(|c| c.value == cell_value)
    }

    fn is_special_case(&self, cells: Vec<Cell>) -> bool {
        let non_empty_count = cells
            .clone()
            .into_iter()
            .filter(|c| c.value != Empty)
            .count();

        // Special case won't happen if not exactly 3 played
        if non_empty_count != 3 {
            return false;
        }

        // If these are in the cells list then special case has occurred
        let expected_cells = vec![
            Cell {
                row: 0,
                column: 0,
                value: CellValue::CPU,
            },
            Cell {
                row: 1,
                column: 1,
                value: CellValue::Human,
            },
            Cell {
                row: 2,
                column: 2,
                value: CellValue::Human,
            },
        ];

        let actual_cells = cells
            .clone()
            .into_iter()
            .filter(|c| expected_cells.contains(c))
            .collect::<Vec<Cell>>();

        expected_cells.eq(&actual_cells)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate::board::Cell;
    use crate::board::CellValue::{Empty, Human, CPU};
    use crate::evaluator::Evaluator;

    #[test]
    fn test_eval_diagonal_win__when__there_is_a_win() {
        let evaluator = Evaluator {};
        let first_diagonal: Vec<(u8, u8)> =
            vec![(0 as u8, 0 as u8), (1 as u8, 1 as u8), (2 as u8, 2 as u8)];
        let human_win_cell_1 = Cell {
            row: 0,
            column: 0,
            value: Human,
        };
        let mut human_win_cell_2 = human_win_cell_1.clone();
        human_win_cell_2.row = 1;
        human_win_cell_2.column = 1;
        let mut human_win_cell_3 = human_win_cell_1.clone();
        human_win_cell_3.row = 2;
        human_win_cell_3.column = 2;
        let cells = vec![human_win_cell_1, human_win_cell_2, human_win_cell_3];

        let actual = evaluator.eval_diagonal_win(cells, first_diagonal, Human);
        let expected = true;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_evaluate__when__human_row_win() {
        let evaluator = Evaluator {};
        let cells = vec![(0, 0), (0, 1), (0, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Human,
            })
            .collect();

        let actual = evaluator.evaluate(cells);
        let expected = 10;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_evaluate__when__cpu_row_win() {
        let evaluator = Evaluator {};
        let cells: Vec<Cell> = vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Empty,
            })
            .collect();
        let win_cells: Vec<Cell> = vec![(2, 0), (2, 1), (2, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: CPU,
            })
            .collect();

        let x = cells.into_iter().chain(win_cells.into_iter()).collect();

        let actual = evaluator.evaluate(x);
        let expected = -10;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_evaluate__when__human_column_win() {
        let evaluator = Evaluator {};
        let cells = vec![(0, 0), (1, 0), (2, 0)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Human,
            })
            .collect();

        let actual = evaluator.evaluate(cells);
        let expected = 10;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_evaluate__when__cpu_column_win() {
        let evaluator = Evaluator {};
        let cells = vec![(0, 0), (1, 0), (2, 0)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: CPU,
            })
            .collect();

        let actual = evaluator.evaluate(cells);
        let expected = -10;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_evaluate__when__human_diagonal_1_win() {
        let evaluator = Evaluator {};
        let mut cells: Vec<Cell> = vec![(0, 2), (2, 0)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Empty,
            })
            .collect();
        let win_cells: Vec<Cell> = vec![(0, 0), (1, 1), (2, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Human,
            })
            .collect();
        cells.extend(win_cells.into_iter());

        let actual = evaluator.evaluate(cells);
        let expected = 10;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_evaluate__when__human_diagonal_2_win() {
        let evaluator = Evaluator {};
        let mut cells: Vec<Cell> = vec![(0, 0), (2, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Empty,
            })
            .collect();
        let win_cells: Vec<Cell> = vec![(0, 2), (1, 1), (2, 0)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Human,
            })
            .collect();
        cells.extend(win_cells.into_iter());

        let actual = evaluator.evaluate(cells);
        let expected = 10;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_evaluate__when__cpu_diagonal_1_win() {
        let evaluator = Evaluator {};
        let mut cells: Vec<Cell> = vec![(0, 2), (2, 0)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Empty,
            })
            .collect();
        let win_cells: Vec<Cell> = vec![(0, 0), (1, 1), (2, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: CPU,
            })
            .collect();
        cells.extend(win_cells.into_iter());

        let actual = evaluator.evaluate(cells);
        let expected = -10;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_evaluate__when__cpu_diagonal_2_win() {
        let evaluator = Evaluator {};
        let mut cells: Vec<Cell> = vec![(0, 0), (2, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Empty,
            })
            .collect();
        let win_cells: Vec<Cell> = vec![(0, 2), (1, 1), (2, 0)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: CPU,
            })
            .collect();
        cells.extend(win_cells.into_iter());

        let actual = evaluator.evaluate(cells);
        let expected = -10;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_evaluate__when__no_win() {
        let evaluator = Evaluator {};
        let cells = vec![(0, 0), (0, 1), (0, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Empty,
            })
            .collect();

        let actual = evaluator.evaluate(cells);
        let expected = 0;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_minimax__when__player_won() {
        let mut evaluator = Evaluator {};
        let cells = vec![(0, 0), (0, 1), (0, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Human,
            })
            .collect();

        let actual = evaluator.minimax(cells, 0);
        let expected = 10;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_minimax__when__cpu_won() {
        let mut evaluator = Evaluator {};
        let cells = vec![(0, 0), (0, 1), (0, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: CPU,
            })
            .collect();

        let actual = evaluator.minimax(cells, 0);
        let expected = -10;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_minimax__when__no_moves_left() {
        let mut evaluator = Evaluator {};
        let mut cpu_cells: Vec<Cell> = vec![(0, 0), (0, 2), (1, 0), (1, 2), (2, 1)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: CPU,
            })
            .collect();

        let human_cells: Vec<Cell> = vec![(0, 1), (1, 1), (2, 0), (2, 2)]
            .into_iter()
            .map(|(row, column)| Cell {
                row,
                column,
                value: Human,
            })
            .collect();

        cpu_cells.extend(human_cells.into_iter());

        let actual = evaluator.minimax(cpu_cells, 0);
        let expected = 0;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_best_move__when__middle_cell_available() {
        let mut evaluator = Evaluator {};
        let cells: Vec<Cell> = vec![
            (0, 0, Human),
            (0, 1, Empty),
            (0, 2, Empty),
            (1, 0, Empty),
            (1, 1, Empty),
            (1, 2, Empty),
            (2, 0, Empty),
            (2, 1, Empty),
            (2, 2, Empty),
        ]
        .into_iter()
        .map(|(row, column, value)| Cell { row, column, value })
        .collect();

        let actual = evaluator.find_best_move(cells);
        let expected = Cell {
            row: 1,
            column: 1,
            value: CPU,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_best_move__when__diagonal_loss_at_risk() {
        let mut evaluator = Evaluator {};
        let cells: Vec<Cell> = vec![
            (0, 0, Human),
            (0, 1, Empty),
            (0, 2, Empty),
            (1, 0, Empty),
            (1, 1, Human), // Can't be new as it's prioritised
            (1, 2, Empty),
            (2, 0, Empty),
            (2, 1, Empty),
            (2, 2, Empty),
        ]
        .into_iter()
        .map(|(row, column, value)| Cell { row, column, value })
        .collect();

        let actual = evaluator.find_best_move(cells);
        let expected = Cell {
            row: 2,
            column: 2,
            value: CPU,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_best_move__when__row_loss_at_risk() {
        let mut evaluator = Evaluator {};
        let cells: Vec<Cell> = vec![
            (0, 0, Human),
            (0, 1, Human),
            (0, 2, Empty),
            (1, 0, Empty),
            (1, 1, Human), // Can't be new as it's prioritised
            (1, 2, Empty),
            (2, 0, Empty),
            (2, 1, Empty),
            (2, 2, Empty),
        ]
        .into_iter()
        .map(|(row, column, value)| Cell { row, column, value })
        .collect();

        let actual = evaluator.find_best_move(cells);
        let expected = Cell {
            row: 0,
            column: 2,
            value: CPU,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_best_move__when__column_loss_at_risk() {
        let mut evaluator = Evaluator {};
        let cells: Vec<Cell> = vec![
            (0, 0, Empty),
            (0, 1, Human),
            (0, 2, Empty),
            (1, 0, Empty),
            (1, 1, Human),
            (1, 2, Empty),
            (2, 0, Empty),
            (2, 1, Empty),
            (2, 2, Empty),
        ]
        .into_iter()
        .map(|(row, column, value)| Cell { row, column, value })
        .collect();

        let actual = evaluator.find_best_move(cells);
        let expected = Cell {
            row: 2,
            column: 1,
            value: CPU,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_best_move__handle_special_case() {
        let mut evaluator = Evaluator {};
        let cells: Vec<Cell> = vec![
            (0, 0, CPU),
            (0, 1, Empty),
            (0, 2, Empty),
            (1, 0, Empty),
            (1, 1, Human),
            (1, 2, Empty),
            (2, 0, Empty),
            (2, 1, Empty),
            (2, 2, Human),
        ]
        .into_iter()
        .map(|(row, column, value)| Cell { row, column, value })
        .collect();

        let actual = evaluator.find_best_move(cells);
        let expected = Cell {
            row: 0,
            column: 2,
            value: CPU,
        };

        assert_eq!(actual, expected);
    }
}
