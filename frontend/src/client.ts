import { DefaultRemoteGame, RemoteGame } from "./game";
import axios from "axios";

export type GameConfig = {
  boardSize: number;
  playerLimit: number;
};

export class Client {
  async newGame(gameConfig: GameConfig, name: string): Promise<RemoteGame> {
    const resp = await axios.post("http://localhost:8080/api/create-game", {
      board_size: gameConfig.boardSize,
      player_limit: gameConfig.playerLimit,
    });

    const { code } = await resp.data;
    return await this.joinGame(code, name);
  }

  async joinGame(gameCode: string, player_name: string): Promise<RemoteGame> {
    const ws = new WebSocket(
      "ws://localhost:8080/api/join-game?" +
        new URLSearchParams({ code: gameCode, player_name: player_name })
    );

    return new DefaultRemoteGame(ws);
  }
}
