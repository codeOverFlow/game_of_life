use crate::automata::cellul::Cellul;
use crate::automata::cellul::CellulState;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
/// The grid of the game.
pub struct Grid {
    /// All the cells.
    pub cells: Vec<Cellul>,
    /// The width of the grid.
    width: u32,
    /// The height of the grid.
    height: u32,
}

impl Grid {
    /// Create a new grid.
    ///
    /// # Arguments
    ///
    /// * `width` - An unsigned int representing the width of the grid.
    /// * `height` - An unsigned int representing the height of the grid.
    /// # Examples
    ///
    /// ```
    /// use game_of_life::automata::grid::Grid;
    /// let grid = Grid::new(5, 5);
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        let mut cells: Vec<Cellul> = Vec::new();
        for r in 0..height {
            for c in 0..width {
                cells.push(Cellul::from((r, c)));
            }
        }
        Grid {
            cells,
            width,
            height,
        }
    }

    fn compute_index(&self, row: u32, column: u32) -> usize {
        (row * self.height + column) as usize
    }

    pub fn get_cellul(&self, row: u32, column: u32) -> Option<&Cellul> {
        self.cells.get(self.compute_index(row, column))
    }

    pub fn update_grid(self: Grid) -> Grid {
        let width = self.width;
        let height = self.height;
        let new_grid = Grid::from(&self);
        for row in 0..width {
            for column in 0..height {
                if let Some(cell) = self.get_cellul(row, column) {
                    let neighbors_coord = cell.coordinate.get_neighboors_coords(width, height);

                    let neighbors_cells = neighbors_coord
                        .iter()
                        .map(|coord| self.get_cellul(coord.row, coord.column));

                    let alive_neighbors = neighbors_cells
                        .filter(|option_c| match option_c {
                            Some(c) => c.is_alive(),
                            None => false,
                        })
                        .count();

                    // update the cell in the new grid
                    // 1. Die from solitude or overpopulation
                    if cell.is_alive() && (alive_neighbors <= 1 || alive_neighbors >= 4) {
                        if let Some(new_cell) =
                            new_grid.get_cellul(cell.coordinate.row, cell.coordinate.column)
                        {
                            new_cell.update_state(CellulState::DEAD)
                        }
                    }
                    if !cell.is_alive() && alive_neighbors == 3 {
                        if let Some(new_cell) =
                            new_grid.get_cellul(cell.coordinate.row, cell.coordinate.column)
                        {
                            new_cell.update_state(CellulState::ALIVE)
                        }
                    }
                }
            }
        }
        new_grid
    }
}

impl From<&Grid> for Grid {
    fn from(grid: &Grid) -> Self {
        let new_grid = Grid::new(grid.width, grid.height);
        for cell in &grid.cells {
            if let Some(c) = new_grid.get_cellul(cell.coordinate.row, cell.coordinate.column) {
                c.update_state(cell.get_state());
            }
        }
        new_grid
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut counter = 0;
        for cell in self.cells.iter() {
            if counter == self.width {
                counter = 0;
                write!(f, "\n")?;
            }
            match cell.is_alive() {
                true => write!(f, "1")?,
                false => write!(f, "0")?,
            };
            counter += 1;
        }
        Ok(())
    }
}
