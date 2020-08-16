use crate::automata::cellul::CellulState::{ALIVE, DEAD};
use std::cell::Cell;

/////////////////////////////////////////////
// COORDINATE STRUCTURE
/////////////////////////////////////////////

#[derive(Debug, Copy, Clone, Eq)]
/// The coordinate of a `Cellul`.
pub struct Coordinate {
    /// The row number.
    pub row: u32,
    /// The column number.
    pub column: u32,
}

impl Coordinate {
    /// Create a new `Coordinate`.
    ///
    /// # Arguments
    ///
    /// * `row` - An unsigned int representing the row number.
    /// * `column` - An unsigned int representing the column number.
    ///
    /// # Examples
    ///
    /// ```
    /// use game_of_life::automata::cellul::Coordinate;
    /// let coordinate = Coordinate::new(5, 5);
    /// ```
    pub fn new(row: u32, column: u32) -> Coordinate {
        Coordinate { row, column }
    }

    /// Check if the `Coordinate` is valid depending on the size of the `Grid`.
    ///
    /// # Arguments
    ///
    /// * `grid_width` - An unsigned int representing the width of the grid.
    /// * `grid_height` - An unsigned int representing the height of the grid.
    pub fn is_valid(row: i32, column: i32, grid_width: u32, grid_height: u32) -> bool {
        column >= 0 && column < grid_width as i32 && row >= 0 && row < grid_height as i32
    }

    pub fn get_neighboors_coords(&self, grid_width: u32, grid_height: u32) -> Vec<Coordinate> {
        let mut coords: Vec<Coordinate> = Vec::new();
        // from -1 to 1
        for offset_y in -1..=1 {
            for offset_x in -1..=1 {
                // compute new row
                let new_row = (self.row as i32) + offset_y;
                // compute new column
                let new_column = (self.column as i32) + offset_x;

                // create new coordinate
                if Coordinate::is_valid(new_row, new_column, grid_width, grid_height) {
                    let coord = Coordinate::new(new_row as u32, new_column as u32);
                    if coord != *self {
                        coords.push(coord);
                    }
                }
            }
        }
        coords
    }
}

impl From<(u32, u32)> for Coordinate {
    fn from(tuple: (u32, u32)) -> Coordinate {
        Coordinate::new(tuple.0, tuple.1)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        self.row == other.row && self.column == other.column
    }
}

/////////////////////////////////////////////
// CELLULSTATE ENUM
/////////////////////////////////////////////

/// The state of the `Cellul`.
#[derive(Debug, Copy, Clone)]
pub enum CellulState {
    /// The `Cellul` is alive.
    ALIVE,
    /// The `Cellul` is dead.
    DEAD,
}

/////////////////////////////////////////////
// CELLUL STRUCTURE
/////////////////////////////////////////////

#[derive(Debug)]
pub struct Cellul {
    state: Cell<CellulState>,
    pub coordinate: Coordinate,
}

impl Cellul {
    pub fn get_state(&self) -> CellulState {
        self.state.get()
    }

    pub fn is_alive(&self) -> bool {
        match self.state.get() {
            ALIVE => true,
            _ => false,
        }
    }

    pub fn update_state(&self, new_state: CellulState) {
        self.state.set(new_state)
    }

    pub fn revive_cellul(&self) {
        self.state.set(ALIVE)
    }

    pub fn kill_cellul(&self) {
        self.state.set(DEAD)
    }
}

impl From<Coordinate> for Cellul {
    fn from(coord: Coordinate) -> Cellul {
        Cellul {
            state: Cell::new(CellulState::DEAD),
            coordinate: coord,
        }
    }
}

impl From<(u32, u32)> for Cellul {
    fn from(coord: (u32, u32)) -> Cellul {
        Cellul {
            state: Cell::new(CellulState::DEAD),
            coordinate: Coordinate::from(coord),
        }
    }
}
