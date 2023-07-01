import { DefaultRemoteGame, RemoteGame } from "./game";
import axios from "axios";

export type GameConfig = {
  boardSize: number;
  playersLimit: number;
};

export class Client {
  async newGame(gameConfig: GameConfig, name: string): Promise<RemoteGame> {
    const resp = await axios.post("http://localhost:8080/api/create-game", {
      board_size: gameConfig.boardSize,
      players_limit: gameConfig.playersLimit,
    });

    const { code } = await resp.data;
    return await this.joinGame(code, name);
  }

  async joinGame(gameCode: string, name: string): Promise<RemoteGame> {
    const ws = new WebSocket(
      "ws://localhost:8080/api/join-game?" +
        new URLSearchParams({ code: gameCode, name: name })
    );

    return new DefaultRemoteGame(ws);
  }
}
