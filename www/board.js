import {Game} from "wasm-impossible-tic-tac-toe";

const game = Game.new();

// If game state is draw or CPU won player should not be able to make any more plays
let isPlayable = true;

const updateBoard = () => {
  game.getCells().forEach(c => {
    const {row, column} = c;
    const element = document.getElementById(`cell-${row}-${column}`);
    element.innerText = toBoardValue(c.value);
  });

  if (game.isCpuWinner()) {
    document.querySelector("#lose-text").style.display = "inline-block";
    isPlayable = false;
  } else if (!game.hasEmptyCells()) {
    document.querySelector("#draw-text").style.display = "inline-block";
    isPlayable = false;
  }
};

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
    if (!isPlayable) {
      return;
    }

    if (e.target.innerText === "") {
      game.humanPlay(x, y);
      game.hasEmptyCells() && game.cpuPlay();
      updateBoard();
    }
  };
};
const initGame = () => {
  game.getCells()
    .map(toObj)
    .map(getElement)
    .map(attachOnClick);

  const resetBoardState = () => {
    isPlayable = true;
    document.querySelector("#draw-text").style.display = "none";
    document.querySelector("#lose-text").style.display = "none";
    game.restart();
    updateBoard();
  };

  document.querySelector("#btn-restart").onclick = () => resetBoardState();
  document.addEventListener("keypress", e => e.key.toLowerCase() === "r" && resetBoardState());
};

initGame();
