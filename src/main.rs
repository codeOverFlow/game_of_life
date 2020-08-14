mod automata;

use crate::automata::cellul::CellulState;
use crate::automata::grid::Grid;

fn main() {
    let grid = Grid::new(5, 5);
    let v: Vec<(u32, u32)> = vec![(2, 1), (2, 2), (2, 3)];
    for (r, c) in v {
        if let Some(cell) = grid.get_cellul(r, c) {
            cell.update_state(CellulState::ALIVE);
        }
    }
    println!("{}\n\n", grid);
    let new_grid = grid.update_grid();
    println!("{}\n\n", new_grid);

    let new_grid = new_grid.update_grid();
    println!("{}\n\n", new_grid);

    let new_grid = new_grid.update_grid();
    println!("{}\n\n", new_grid);

    let new_grid = new_grid.update_grid();
    println!("{}\n\n", new_grid);

    let new_grid = new_grid.update_grid();
    println!("{}\n\n", new_grid);

    let new_grid = new_grid.update_grid();
    println!("{}\n\n", new_grid);
}
