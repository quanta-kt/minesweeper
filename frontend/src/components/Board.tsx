import { Cell, GameState } from "../game";

interface CellViewProps {
  index: number;
  cell: Cell;
  onReveal: (index: number) => void;
  onToggleFlag: (index: number) => void;
}

interface BoardRowProps {
  rowIndex: number;
  boardState: GameState;
  onReveal: (index: number) => void;
  onToggleFlag: (index: number) => void;
}

interface BoardProps {
  state: GameState;
  onReveal: (index: number) => void;
  onToggleFlag: (index: number) => void;
}

export function Board({ state, onReveal, onToggleFlag }: BoardProps) {
  const rows = new Array(state.board_size)
    .fill(null)
    .map((_, rowIndex) => (
      <BoardRow
        key={rowIndex}
        boardState={state}
        rowIndex={rowIndex}
        onReveal={onReveal}
        onToggleFlag={onToggleFlag}
      />
    ));

  return <table className="board">{rows}</table>;
}

function BoardRow({
  rowIndex: index,
  boardState,
  onReveal,
  onToggleFlag,
}: BoardRowProps) {
  const start = index * boardState.board_size;

  const cells = boardState.board_state
    .slice(start, start + boardState.board_size)
    .map((cell, i) => (
      <td>
        <CellView
          index={start + i}
          cell={cell}
          onReveal={onReveal}
          onToggleFlag={onToggleFlag}
        />
      </td>
    ));

  return <tr>{cells}</tr>;
}

function CellView({ index, cell, onReveal, onToggleFlag }: CellViewProps) {
  let text = "";

  switch (cell.state) {
    case "number":
      text = cell.value === 0 ? " " : cell.value!!.toString();
      break;
    case "flagged":
      text = "🚩";
      break;
    case "mine":
      text = "💣";
      break;
    case "unrevealed":
      text = "❓";
      break;
  }

  return (
    <button
      className="cell"
      onMouseDown={(e) => {
        if (e.button == 0) {
          if (cell.state !== "unrevealed") return;
          onReveal(index);
          e.preventDefault();
        } else if (e.button == 2) {
          if (cell.state !== "unrevealed" && cell.state !== "flagged") return;
          onToggleFlag(index);
          e.preventDefault();
        }
      }}
      onContextMenu={(e) => {
        e.preventDefault();
        return false;
      }}
    >
      {text}
    </button>
  );
}
