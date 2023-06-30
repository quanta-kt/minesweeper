import { useEffect, useState } from "react";
import "./App.css";
import { GameState, RemoteGame, DefaultRemoteGame } from "./game";
import { Board } from "./components/Board";

const game: RemoteGame = new DefaultRemoteGame();

function App() {
  const [board, setBoard] = useState<GameState | null>(null);

  useEffect(() => {
    (async () => {
      setBoard(await game.getBoard());
    })();
  }, []);

  const onReveal = (index: number) => {
    game.reveal(index).then(() =>
      game.getBoard().then((board) => {
        setBoard(board);
      })
    );
  };

  const onToggleFlag = async (index: number) => {
    game.toggleFlag(index).then(() =>
      game.getBoard().then((board) => {
        setBoard(board);
      })
    );
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

export default App;
