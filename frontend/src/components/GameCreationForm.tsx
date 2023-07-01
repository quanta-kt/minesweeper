import { useState } from "react";
import { GameConfig } from "../client";

type GameCreationFromProps = {
  onRequestGame: (config: GameConfig, name: string) => void;
};

export default function GameCreationFrom({
  onRequestGame,
}: GameCreationFromProps) {
  const [name, setName] = useState("");
  const [size, setSize] = useState(8);

  return (
    <>
      <div className="new-game-form-container">
        <h2>New Game</h2>
        <div className="new-game-form">
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Your name"
          />
          <input
            type="number"
            value={size}
            min={4}
            max={64}
            onChange={(e) => setSize(Number.parseInt(e.target.value))}
            placeholder="Board size"
          />

          <button
            onClick={() =>
              onRequestGame({ boardSize: size, playersLimit: 8 }, name)
            }
          >
            Start
          </button>
        </div>
      </div>
    </>
  );
}
