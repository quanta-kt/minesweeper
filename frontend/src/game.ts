export type CellState = "number" | "mine" | "flagged" | "unrevealed";

export interface Cell {
  state: CellState;
  value?: number;
}

export interface GameState {
  board_state: Array<Cell>;
  board_size: number;
}

export interface RemoteGame {
  size: number;
  onBoardUpdated: (listener: (GameState: GameState) => void) => void;
  getBoard: () => GameState | null;
  reveal: (index: number) => void;
  toggleFlag: (index: number) => void;
}

export class DefaultRemoteGame implements RemoteGame {
  size: number;
  ws: WebSocket;
  gameStateListener: ((GameState: GameState) => void) | null;
  _last_known_state: GameState | null;

  constructor(ws: WebSocket) {
    this.size = 8;
    this.ws = ws;
    this.gameStateListener = null;
    this._last_known_state = null;

    ws.addEventListener("message", (event) => {
      const updatedState = JSON.parse(event.data) as GameState;

      const listener = this.gameStateListener;

      if (listener != null) {
        listener(updatedState);
      }
    });
  }

  onBoardUpdated(listener: (gameState: GameState) => void) {
    this.gameStateListener = listener;
  }

  getBoard() {
    return this._last_known_state;
  }

  reveal(index: number) {
    console.log(index);
    this.ws.send(
      JSON.stringify({
        action: "reveal",
        index: index,
      })
    );
  }

  toggleFlag(index: number) {
    this.ws.send(
      JSON.stringify({
        action: "flag",
        index: index,
      })
    );
  }
}
