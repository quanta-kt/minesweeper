import { useEffect, useState } from "react";
import { GameState, RemoteGame } from "../game";
import { Board } from "./Board";

type GameViewProps = {
  game: RemoteGame;
};

export default function GameView({ game }: GameViewProps) {
  const [board, setBoard] = useState<GameState | null>(null);

  useEffect(() => {
    game.onBoardUpdated((board) => setBoard(board));
  }, []);

  const onReveal = (index: number) => {
    console.log("rev");
    game.reveal(index);
  };

  const onToggleFlag = (index: number) => {
    game.toggleFlag(index);
  };

  return (
    <>
      <div className="board-container">
        {board === null ? (
          <></>
        ) : (
          <Board
            state={board}
            onReveal={onReveal}
            onToggleFlag={onToggleFlag}
          ></Board>
        )}
      </div>
    </>
  );
}
