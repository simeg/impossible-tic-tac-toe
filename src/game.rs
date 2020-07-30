use crate::board::Board;
use crate::evaluator::Evaluator;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct Game {
    board: Board,
    evaluator: Evaluator,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Game {
        Game {
            board: Board::default(),
            evaluator: Evaluator {},
        }
    }

    #[wasm_bindgen(js_name = getCells)]
    pub fn get_cells(&self) -> JsValue {
        JsValue::from_serde(&self.board.get_cells()).unwrap()
    }

    #[wasm_bindgen(js_name = isCpuWinner)]
    pub fn is_cpu_winner(&self) -> bool {
        self.board.is_cpu_winner()
    }

    #[wasm_bindgen(js_name = hasEmptyCells)]
    pub fn has_empty_cells(&self) -> bool {
        Board::is_moves_left(self.board.get_cells())
    }

    #[wasm_bindgen(js_name = humanPlay)]
    pub fn human_play(&mut self, row: u8, column: u8) {
        self.board.set_human(row, column);
    }

    #[wasm_bindgen(js_name = cpuPlay)]
    pub fn cpu_play(&mut self) {
        let best_move = self.evaluator.find_best_move(self.board.get_cells());
        self.board.set_cpu(best_move.row, best_move.column);
    }

    pub fn restart(&mut self) {
        self.board.clear();
    }
}
