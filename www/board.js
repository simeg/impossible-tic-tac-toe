import {Game} from "wasm-impossible-tic-tac-toe";

const game = Game.new();

const updateBoard = () =>
  game.getCells().forEach(c => {
    const {row, column} = c;
    const element = document.getElementById(`cell-${row}-${column}`);
    element.innerText = toBoardValue(c.value);
  });

const toBoardValue = rustValue => {
  switch (rustValue.toLowerCase()) {
  case "human":
    return "X";
  case "cpu":
    return "O";
  case "empty":
    return "";
  default:
    throw new Error("Unsupported type: " + rustValue);
  }
};

const toObj = c => ({"id": `cell-${c.row}-${c.column}`, "x": c.row, "y": c.column});
const getElement = c => ({element: document.getElementById(c.id), x: c.x, y: c.y});
const attachOnClick = ({element, x, y}) => {
  element.onclick = e => {
    console.log("[js] clicked");
    if (e.target.innerText === "") {
      game.humanPlay(x, y);
      game.hasEmptyCells() && game.cpuPlay();
      updateBoard();
    }
  };
};
const initGame = () => {
  game.getCells().map(toObj).map(getElement).map(attachOnClick);
  document.querySelector("#btn-restart").onclick = () => {
    game.restart();
    updateBoard();
  };
};

initGame();
