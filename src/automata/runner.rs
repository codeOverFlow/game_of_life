use crate::automata::cellul::Coordinate;
use crate::automata::grid::Grid;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Runner {
    grid: Grid,
    max_step: u32,
    step_timer: Duration,
}

impl Runner {
    pub fn new(grid_width: u32, grid_height: u32, max_step: u32, step_timer: Duration) -> Runner {
        Runner {
            grid: Grid::new(grid_width, grid_height),
            max_step,
            step_timer,
        }
    }

    pub fn initialize_grid(&self, alive_cells: Vec<(u32, u32)>) {
        for (row, column) in alive_cells {
            if Coordinate::is_valid(row as i32, column as i32, self.grid.width, self.grid.height) {
                if let Some(cell) = self.grid.get_cellul(row, column) {
                    cell.revive_cellul();
                }
            }
        }
    }

    fn clear_command(&self, command: &str) {
        if let Ok(status) = Command::new(command.to_string()).status() {
            if !status.success() {
                println!("\n\n")
            }
        } else {
            println!("\n\n")
        }
    }

    #[cfg(unix)]
    fn clear_output(&self) {
        self.clear_command("clear")
    }

    #[cfg(windows)]
    fn clear_output(&self) {
        self.clear_command("cls")
    }

    pub fn run(&self) {
        for _ in 0..self.max_step {
            self.clear_output();
            println!("{}", self.grid);
            thread::sleep(self.step_timer);
            self.grid.update_grid();
        }
    }
}
