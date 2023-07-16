use std::collections::VecDeque;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum CellType {
    Mine,
    Number,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CellState {
    Unrevealed,
    Flagged,
    Revealed,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub index: usize,
    pub cell_type: CellType,
    pub state: CellState,
}

#[derive(Clone, Debug)]
pub struct Board {
    size: usize,
    data: Vec<Cell>,
}

#[derive(Debug)]
pub enum Error {
    CoordinatesOutOfBound,
    InvalidMove,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "state")]
pub enum ExternalCell {
    #[serde(rename = "number")]
    Number { value: u8 },
    #[serde(rename = "mine")]
    Mine,
    #[serde(rename = "flagged")]
    Flagged,
    #[serde(rename = "unrevealed")]
    Unrevealed,
}

impl Board {
    pub fn generate(size: usize) -> Board {
        Board {
            size,
            data: (0..size * size)
                .map(|index| Cell {
                    index,
                    state: CellState::Unrevealed,
                    cell_type: if rand::thread_rng().gen_bool(0.15) {
                        CellType::Mine
                    } else {
                        CellType::Number
                    },
                })
                .collect(),
        }
    }

    pub fn size(self: &Board) -> usize {
        self.size
    }

    pub fn get(self: &Board, index: usize) -> Option<&Cell> {
        self.data.get(index)
    }

    pub fn iter_cells(self: &Board) -> impl Iterator<Item = &Cell> {
        self.data.iter()
    }

    fn iter_surrounding_positions(self: &Board, index: usize) -> impl Iterator<Item = usize> {
        let (row, column) = if let Some(pos) = Self::to_coords(self.size, index) {
            pos
        } else {
            panic!("Position out of bound");
        };

        let surrounding_coordinates = [
            (row.checked_sub(1), column.checked_sub(1)),
            (row.checked_sub(1), Some(column)),
            (row.checked_sub(1), Some(column + 1)),
            (Some(row), column.checked_sub(1)),
            (Some(row), Some(column + 1)),
            (Some(row + 1), column.checked_sub(1)),
            (Some(row + 1), Some(column)),
            (Some(row + 1), Some(column + 1)),
        ];

        let size = self.size();

        surrounding_coordinates
            .into_iter()
            .filter_map(|(row, column)| row.map(|row| column.map(|column| (row, column))).flatten())
            .filter_map(move |(row, column)| Self::to_index(size, row, column))
    }

    pub fn iter_surroundings(self: &Board, index: usize) -> impl Iterator<Item = &Cell> {
        self.iter_surrounding_positions(index)
            .filter_map(|index| self.get(index))
    }

    pub fn get_surrounding_mines_count(self: &Board, index: usize) -> u8 {
        let count = self
            .iter_surroundings(index)
            .filter(|cell| cell.cell_type == CellType::Mine)
            .count();

        count as u8
    }

    pub fn toggle_flag(self: &mut Board, index: usize) -> Result<(), Error> {
        let cell = self
            .data
            .get_mut(index)
            .ok_or(Error::CoordinatesOutOfBound)?;

        match cell.state {
            CellState::Unrevealed => cell.state = CellState::Flagged,
            CellState::Flagged => cell.state = CellState::Unrevealed,
            CellState::Revealed => return Err(Error::InvalidMove),
        };

        Ok(())
    }

    pub fn reveal(self: &mut Board, index: usize) -> Result<(), Error> {
        let cell = self
            .data
            .get_mut(index)
            .ok_or(Error::CoordinatesOutOfBound)?;

        cell.state = CellState::Revealed;

        let is_empty = self.get_surrounding_mines_count(index) == 0;
        if !is_empty {
            return Ok(());
        }

        // An empty cell was revealed
        // Perform a BFS to find and reveal all the surrounding empty cells as well
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(index);

        while !queue.is_empty() {
            let index = queue.pop_front().unwrap();

            let cell = self.data.get_mut(index).unwrap();
            cell.state = CellState::Revealed;

            let is_empty =
                cell.cell_type == CellType::Number && self.get_surrounding_mines_count(index) == 0;

            if !is_empty {
                continue;
            }

            for cell in self.iter_surroundings(index) {
                if cell.state == CellState::Unrevealed {
                    queue.push_back(cell.index);
                }
            }
        }

        Ok(())
    }

    /**
     * Returns the state of the game suitable for consumption on client side
     */
    pub fn get_external_state(self: &Board) -> Vec<ExternalCell> {
        self.iter_cells()
            .enumerate()
            .map(|(index, cell)| match cell.state {
                CellState::Unrevealed => ExternalCell::Unrevealed,
                CellState::Flagged => ExternalCell::Flagged,

                CellState::Revealed => match cell.cell_type {
                    CellType::Mine => ExternalCell::Mine,
                    CellType::Number => ExternalCell::Number {
                        value: self.get_surrounding_mines_count(index),
                    },
                },
            })
            .collect::<Vec<ExternalCell>>()
    }

    fn to_index(size: usize, row: usize, column: usize) -> Option<usize> {
        if row >= size || column >= size {
            return None;
        }

        Some(size * row + column)
    }

    fn to_coords(size: usize, index: usize) -> Option<(usize, usize)> {
        if size * size <= index {
            None
        } else {
            Some((index / size, index % size))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flag() {
        let mut board = Board::generate(5);

        let _ = board.toggle_flag(11);

        assert!(board.get(11).unwrap().state == CellState::Flagged);
    }

    #[test]
    fn test_reveal() {
        let mut board = Board::generate(5);

        let _ = board.reveal(21);

        assert!(board.get(21).unwrap().state == CellState::Revealed);
    }

    #[test]
    fn test_get_invalid() {
        let board = Board::generate(5);
        assert!(board.get(51).is_none());
        assert!(board.get(25).is_none());
    }

    #[test]
    fn test_corner_surroundings() {
        let board = Board::generate(8);

        let mut actual = board.iter_surrounding_positions(0).collect::<Vec<usize>>();
        actual.sort();

        assert_eq!(
            board.iter_surrounding_positions(0).collect::<Vec<usize>>(),
            vec![1, 8, 9]
        );
    }

    #[test]
    fn test_surrounding_mines_count() {
        let mut data: Vec<Cell> = (0..16)
            .map(|index| Cell {
                index,
                cell_type: CellType::Number,
                state: CellState::Unrevealed,
            })
            .collect();

        data[0] = Cell {
            index: 0,
            cell_type: CellType::Mine,
            state: CellState::Unrevealed,
        };

        let board = Board { data, size: 4 };

        assert_eq!(board.get_surrounding_mines_count(1), 1);
        assert_eq!(board.get_surrounding_mines_count(4), 1);
        assert_eq!(board.get_surrounding_mines_count(2), 0);
        assert_eq!(board.get_surrounding_mines_count(0), 0);
    }
}
