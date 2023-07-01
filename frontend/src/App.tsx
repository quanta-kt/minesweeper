import { useState } from "react";
import "./App.css";
import { RemoteGame } from "./game";
import { Client, GameConfig } from "./client";
import GameCreationFrom from "./components/GameCreationForm";
import GameView from "./components/GameView";

function App() {
  const client = new Client();
  const [connectedGame, setConnectedGame] = useState<RemoteGame | null>(null);

  const createGame = async (gameConfig: GameConfig, name: string) => {
    const game = await client.newGame(gameConfig, name);
    setConnectedGame(game);
  };

  return (
    <>
      {connectedGame !== null ? (
        <GameView game={connectedGame} />
      ) : (
        <GameCreationFrom
          onRequestGame={async (config, name) => await createGame(config, name)}
        />
      )}
    </>
  );
}

export default App;
