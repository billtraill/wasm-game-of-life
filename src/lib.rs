extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

mod utils;
mod web;

use wasm_bindgen::prelude::*;
use std::fmt;
extern crate js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

// Default entry point for WASM called by default. 
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    return web::start();
}


/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
    
        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };
    
        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };
    
        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };
    
        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };
    
        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;
    
        let n = self.get_index(north, column);
        count += self.cells[n] as u8;
    
        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;
    
        let w = self.get_index(row, west);
        count += self.cells[w] as u8;
    
        let e = self.get_index(row, east);
        count += self.cells[e] as u8;
    
        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;
    
        let s = self.get_index(south, column);
        count += self.cells[s] as u8;
    
        let se = self.get_index(south, east);
        count += self.cells[se] as u8;
    
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                // log!("cell[{}, {}] is initially {:?} and has {} live neighbors",
                //       row,
                //       col,
                //       cell,
                //       live_neighbors
                //   );

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                // randomly set a cell to live every now and then!
                if (live_neighbors > 1) && (js_sys::Math::random() < 0.00001) {
                    next[idx] = Cell::Alive;
                } else {
                    next[idx] = next_cell;
                }

                // log!("    it becomes {:?}", next_cell);
                
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 128;
        let height = 128;

        let cells = (0..width * height)
            .map(|_i| {
                if js_sys::Math::random() > 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    // pub fn render(&self) -> String {
    //     self.to_string()
    // }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn render_webgl(&self) -> Result<(), JsValue> {

        // For each cell that is alive 
        // create webgl triangles to draw the cell

        // default webgl geometry runs run -1,1 so 2 is the length
        // TODO: query the current geometry to find the size rather than assume it.
        let cell_x_size = 2.0_f32/(self.width as f32); 
        let cell_y_size = 2.0_f32/(self.height as f32);

        // Create a vector of all points for each triangle
        // 2 triangles for each cell
        let vertices: Vec<f32> = self.cells.iter().enumerate() // enumetate gives us a a tuple with index and ref to vector
                .filter(|e| *e.1 == Cell::Alive)
                .map(|e| {
                    let idx = e.0;
                    let row = (idx as u32) / self.width;
                    let col = (idx as u32) % self.width;
                    let zero = 0.00_f32;
                    //let grid = 0.01_f32;
                    //let size = 0.0080_f32;
                    let fx0 = (cell_x_size *  (row as f32)) -1.0_f32;
                    let fy0 = (cell_y_size *  (col as f32)) -1.0_f32; // check oriantation of this!!
                    let fx1 = fx0 + cell_x_size;
                    let fy1 = fy0 + cell_y_size;
                    let vert = vec![
                        fx0, fy0, zero, // TODO: convert this to 2D - less data should be faster!
                        fx1, fy0, zero,
                        fx0, fy1, zero,
                
                        fx1, fy1, zero,
                        fx0, fy1, zero,
                        fx1, fy0, zero,
                    ];
                    vert
                })
            .flatten()
            .collect::<Vec<f32>>();

    return web::render(vertices);
    }
}

// Note these functions are not exported to javascript with wasm_bindgen as
// you cannot return a borrowed ref 
impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}


impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}