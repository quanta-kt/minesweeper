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
  getBoard: () => Promise<GameState>;
  reveal: (index: number) => Promise<void>;
  toggleFlag: (index: number) => Promise<void>;
}

export class DefaultRemoteGame implements RemoteGame {
  size: number;

  constructor() {
    this.size = 8;
  }

  async getBoard() {
    const data = await fetch("http://localhost:8080");
    return await data.json();
  }
  async reveal(index: number) {
    await fetch(
      "http://localhost:8080/reveal?" +
        new URLSearchParams({ index: index.toString() }),
      {
        method: "POST",
      }
    );
  }
  async toggleFlag(index: number) {
    await fetch(
      "http://localhost:8080/flag?" +
        new URLSearchParams({ index: index.toString() }),
      {
        method: "POST",
      }
    );
  }
}
