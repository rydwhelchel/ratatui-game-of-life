#![allow(dead_code)]
use std::fmt::Formatter;

use ratatui::widgets::canvas::{Painter, Shape};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub state: State,
}

impl Cell {
    fn new(state: State) -> Self {
        Cell { state }
    }

    fn next_state(&self, neighbors: Vec<Cell>) -> Cell {
        let alive_neighbors: Vec<Cell> = neighbors
            .iter()
            .filter(|x| x.state == State::Alive)
            .map(|x| *x)
            .collect();
        let dead_neighbors: Vec<Cell> = neighbors
            .iter()
            .filter(|x| x.state == State::Dead)
            .map(|x| *x)
            .collect();
        assert!(alive_neighbors.len() + dead_neighbors.len() == neighbors.len());
        match self.state {
            State::Alive => {
                if alive_neighbors.len() < 2 || alive_neighbors.len() > 3 {
                    Cell { state: State::Dead }
                } else {
                    *self
                }
            }
            State::Dead => {
                if alive_neighbors.len() == 3 {
                    Cell {
                        state: State::Alive,
                    }
                } else {
                    *self
                }
            }
        }
    }
}

fn get_neighbors(coords: Coords, game: &GameOfLife) -> Vec<Cell> {
    let mut cells: Vec<Cell> = Vec::new();
    let x = coords.x;
    let y = coords.y;
    let x_not_low = coords.x != 0;
    let y_not_low = coords.y != 0;
    let x_not_high = coords.x != game.board[0].len() - 1;
    let y_not_high = coords.y != game.board.len() - 1;
    if x_not_low {
        if y_not_low {
            cells.push(game.get_cell_at(Coords { x: x - 1, y: y - 1 }));
        }
        if y_not_high {
            cells.push(game.get_cell_at(Coords { x: x - 1, y: y + 1 }));
        }
        cells.push(game.get_cell_at(Coords { x: x - 1, y }));
    }
    if y_not_low {
        if x_not_high {
            cells.push(game.get_cell_at(Coords { x: x + 1, y: y - 1 }));
        }
        cells.push(game.get_cell_at(Coords { x, y: y - 1 }));
    }
    if x_not_high {
        if y_not_high {
            cells.push(game.get_cell_at(Coords { x: x + 1, y: y + 1 }));
        }
        cells.push(game.get_cell_at(Coords { x: x + 1, y }));
    }
    if y_not_high {
        cells.push(game.get_cell_at(Coords { x, y: y + 1 }));
    }
    cells
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameOfLife {
    board: Vec<Vec<Cell>>,
}

impl Shape for GameOfLife {
    fn draw(&self, painter: &mut Painter) {
        for coords in self.get_live_cells() {
            let x = coords.x as f64;
            let y = coords.y as f64;
            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, ratatui::style::Color::Red);
            }
        }
    }
}

impl GameOfLife {
    pub fn new(x: usize, y: usize, live_cells: Vec<Coords>) -> Self {
        let mut new_frame: Vec<Vec<Cell>> = vec![vec![Cell::new(State::Dead); x]; y];
        for cell in live_cells {
            *new_frame.get_mut(cell.y).unwrap().get_mut(cell.x).unwrap() = Cell {
                state: State::Alive,
            };
        }

        GameOfLife { board: new_frame }
    }

    fn max_x(&self) -> usize {
        self.board.get(0).unwrap().len()
    }

    fn max_y(&self) -> usize {
        self.board.len()
    }

    pub fn next_frame(&self) -> GameOfLife {
        let max_x = self.max_x();
        let max_y = self.max_y();
        let mut new_frame: Vec<Vec<Cell>> = vec![vec![Cell::new(State::Dead); max_x]; max_y];
        let mut y = 0;
        while y < 25 {
            let mut x = 0;
            while x < 50 {
                let prev_cell = self.board[y][x];
                new_frame[y][x] = prev_cell.next_state(get_neighbors(Coords { x, y }, self));
                x += 1;
            }
            y += 1;
        }

        GameOfLife { board: new_frame }
    }

    fn get_cell_at(&self, coords: Coords) -> Cell {
        *self.board.get(coords.y).unwrap().get(coords.x).unwrap()
    }

    fn get_live_cells(&self) -> Vec<Coords> {
        let mut live_cells = Vec::new();
        let mut y = 0;
        while y < self.board.len() {
            let mut x = 0;
            while x < self.board.get(0).unwrap().len() {
                let cell = self.get_cell_at(Coords { x, y });
                if cell.state == State::Alive {
                    live_cells.push(Coords { x, y });
                }
                x += 1;
            }
            y += 1;
        }
        live_cells
    }
}

impl std::fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut y = 0;
        while y < self.board.len() {
            let mut x = 0;
            writeln!(f, "").expect("Unable to write");
            while x < self.board[0].len() {
                write!(
                    f,
                    "{}",
                    match self.board[y][x].state {
                        State::Alive => 'o',
                        State::Dead => '.',
                    }
                )
                .expect("Unable to write");
                x += 1;
            }
            y += 1;
        }
        Ok(())
    }
}
