// TODO
#![allow(dead_code)]
#![allow(unused_variables)]

use std::convert::TryFrom;
use strided::Stride;

/// Aquarium puzzle solver
/// https://www.puzzle-aquarium.com/
///
/// > **Aquarium** is a logic puzzle with simple rules and challenging solutions.
///
/// > The rules of Aquarium are simple:
/// > The puzzle is played on a rectangular grid divided into blocks called "aquariums".
/// > You have to "fill" the aquariums with water up to a certain level or leave it empty.
/// > The water level in each aquarium is one and the same across its full width
/// > The numbers outside the grid show the number of filled cells horizontally and vertically.
///
/// An aquarium puzzle is defined by an NxM (width x height) grid of cells. (Commonly NxN).
/// Between each cell, there may be a seperator (wall or floor) that prevents the cells from touching.
/// Cells that touch are in the same partition.
/// Cells can be marked:
/// * Empty: Logically unknown required state
/// * Flooded: Known to be required to be flooded
/// * Invalid: Known to be incapable of being flooded
///
/// Every cell in the same partition and row must have the same state.
/// Every cell in the same partition and above an invalid cell must be invalid.
/// Every cell in the same partition and below a  flooded cell must be flooded.
///
///
/// Note: Cells can be in the same row and partition without directly touching. i.e. a 'U' shape:
/// ```
/// 0 1 0  // The '0's don't touch but are in the same partition and therefore must have the same state.
/// 0 0 0
/// ```

//

/// Valid transitions: Empty->Flooded, Empty->Invalid
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum CellState {
    Empty,
    Flooded,
    Invalid,
}

impl CellState {
    fn rep(&self) -> char {
        match self {
            CellState::Empty => ' ',
            CellState::Flooded => '*',
            CellState::Invalid => 'X',
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum WallState {
    None,
    Wall,
}

impl WallState {
    fn rep(&self) -> char {
        use WallState::*;
        match self {
            None => '|',
            Wall => '#',
        }
    }

    fn from_bool(state: bool) -> WallState {
        match state {
            true => WallState::Wall,
            false => WallState::None,
        }
    }

    fn rep_bool(state: bool) -> char {
        WallState::from_bool(state).rep()
    }
}

#[derive(PartialEq, Copy, Clone)]
enum FloorState {
    None,
    Bott,
}

impl FloorState {
    fn rep(&self) -> char {
        use FloorState::*;
        match self {
            None => '-',
            Bott => '#',
        }
    }

    fn from_bool(state: bool) -> FloorState {
        match state {
            true => FloorState::Bott,
            false => FloorState::None,
        }
    }

    fn rep_bool(state: bool) -> char {
        FloorState::from_bool(state).rep()
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Cell {
    state: CellState,
    partition: isize,
// Could include the board cooridnate, but I don't think I want that. 
}

impl Cell {
    fn rep(&self) -> char {
        self.state.rep()
    }
}

struct Board {
    // Visual properties of the board
    width: usize,
    height: usize,
    // width x height
    cells: Vec<Cell>,
    // height
    row_hints: Vec<isize>,
    // width
    col_hints: Vec<isize>,
}

impl Board {

    fn cell_at(&self, ix: usize, iy: usize) -> Cell {
        assert!(ix < self.width && iy < self.height);
        let row_offset = iy * self.width;
        self.cells[row_offset + ix]
    }

    fn cell_state_at(&self, ix: usize, iy: usize) -> CellState {
        assert!(ix < self.width && iy < self.height);
        let row_offset = iy * self.width;
        self.cells[row_offset + ix].state
    }

    fn set_cell_at(&mut self, ix: usize, iy: usize, state: CellState) {
        assert!(ix < self.width && iy < self.height);
        let row_offset = iy * self.width;
        self.cells[row_offset + ix].state = state;
    }

    fn partition_at(&self, ix: usize, iy: usize) -> isize {
        assert!(ix < self.width && iy < self.height);
        let row_offset = iy * self.width;
        self.cells[row_offset + ix].partition
    }

    //---
    //
    fn wall_at(&self, ix: usize, iy: usize) -> bool {
        assert!(ix + 1 < self.width && iy < self.height);
        self.partition_at(ix, iy) != self.partition_at(ix + 1, iy)
    }
    //
    fn floor_at(&self, ix: usize, iy: usize) -> bool {
        assert!(ix < self.width && iy + 1 < self.height);
        self.partition_at(ix, iy) != self.partition_at(ix, iy + 1)
    }

    // TODO
    fn make(width: usize, height: usize) -> Board {
        let board = Board {
            width,
            height,
            cells: vec![
                Cell {
                    state: CellState::Empty,
                    partition: -1
                };
                width * height
            ],
            row_hints: vec![0; height],
            col_hints: vec![0; width],
        };
        board
    }

    fn make_b0() -> Board {
        // 6x6 Easy ID: 3,095,209 https://www.puzzle-aquarium.com/specfic.php

        let width = 6;
        let height = 6;
        let count = width * height;
        let partitions = vec![
            00, 00, 00, 00, 01, 01, //
            00, 00, 02, 02, 01, 01, //
            03, 00, 03, 02, 04, 05, //
            03, 03, 03, 02, 04, 05, //
            03, 03, 03, 03, 03, 05, //
            03, 03, 05, 05, 05, 05,
        ];

        let cells: Vec<_> = partitions
            .iter()
            .map(|&partition| Cell {
                state: CellState::Empty,
                partition,
            })
            .collect();

        let board = Board {
            width,
            height,
            cells,
            row_hints: vec![2, 4, 3, 2, 1, 4],
            col_hints: vec![1, 2, 1, 3, 5, 4],
        };

        board
    }

    fn make_b0_solved() -> Board {
        let mut board = Board::make_b0();
        use CellState::*;

        let states = vec![
            Invalid, Invalid, Invalid, Invalid, Flooded, Flooded, //
            Flooded, Flooded, Invalid, Invalid, Flooded, Flooded, //
            Invalid, Flooded, Invalid, Flooded, Flooded, Invalid, //
            Invalid, Invalid, Invalid, Flooded, Flooded, Invalid, //
            Invalid, Invalid, Invalid, Invalid, Invalid, Flooded, //
            Invalid, Invalid, Flooded, Flooded, Flooded, Flooded,
        ];

        for (cell, state) in board.cells.iter_mut().zip(states) {
            cell.state = state;
        }

        board
    }

    /// Set each cell in the same partition as the cell at (ix, iy)
    /// and the same or lower row (iy) to be flooded
    fn flood(&mut self, ix: usize, iy: usize) {
        let partition = self.partition_at(ix, iy);
        for iy in iy..self.height {
            for ix in 0..self.width {
                if self.partition_at(ix, iy) == partition {
                    self.set_cell_at(ix, iy, CellState::Flooded);
                }
            }
        }
    }

    /// Set each cell in the same partition as the cell at (ix, iy)
    /// and the same or higher row (iy) to be invalid
    fn invalidate(&mut self, ix: usize, iy: usize) {
        let partition = self.partition_at(ix, iy);
        for iy in 0..iy + 1 {
            for ix in 0..self.width {
                if self.partition_at(ix, iy) == partition {
                    self.set_cell_at(ix, iy, CellState::Invalid);
                }
            }
        }
    }

    fn print(&self) {
        // let print_index = true;
        let print_index = false;
        let print_partitions = false;
        // let print_partitions = true;

        // Todo: Use format width for numbers

        // print top
        print!("   ");
        for hint in &self.col_hints {
            print!("{}  ", hint);
        }
        println!();

        print!("  ");
        for _ in 0..self.width * 2 + self.width + 1 {
            print!("#");
        }
        println!();

        //
        for iy in 0..self.height {
            print!("{} #", self.row_hints[iy]);
            let row_cells = {
                let row_offset = iy * self.width;
                &self.cells[row_offset..row_offset + self.width]
            };

            let row_walls: Vec<_> = (0..self.width - 1)
                .map(|ix| WallState::rep_bool(self.wall_at(ix, iy)))
                .collect();

            // Print cells and walls
            for ix in 0..self.width {
                if !print_partitions {
                    print!("{} ", row_cells[ix].rep());
                } else {
                    print!(
                        "{}{}",
                        row_cells[ix].rep(),
                        self.cells[ix + iy * self.width].partition
                    );
                }

                if ix + 1 != self.width {
                    print!("{}", row_walls[ix]);
                }
            }

            let n_row = row_cells
                .iter()
                .filter(|&&cell| cell.state == CellState::Flooded)
                .count();
            let row_remainder = self.row_hints[iy] - isize::try_from(n_row).unwrap();
            print!("# {}", row_remainder);

            // Row index
            if print_index {
                print!(" |  {}", iy);
            }

            println!();
            // Print floors
            if iy + 1 != self.height {
                let row_floor: Vec<_> = (0..self.width).map(|ix| self.floor_at(ix, iy)).collect();

                print!("  #");
                for (ix, it) in row_floor.iter().enumerate() {
                    let rep = FloorState::rep_bool(*it);

                    // Up, Left (this), Right, Down
                    let walled_neighbors = [
                        if ix + 1 == self.width {
                            true
                        } else {
                            self.wall_at(ix, iy)
                        },
                        *it,
                        if ix + 1 == self.width {
                            true
                        } else {
                            self.floor_at(ix + 1, iy)
                        },
                        if iy + 1 == self.height || ix + 1 == self.width {
                            true
                        } else {
                            self.wall_at(ix, iy + 1)
                        },
                    ];
                    let count = walled_neighbors.iter().filter(|&&x| x).count();
                    // `if any` would have been just as fine I guess
                    let corner = if count >= 2 { '#' } else { '+' };

                    print!("{}{}{}", rep, rep, corner);
                }
                if print_index {
                    print!("   |  ");
                }
                println!();
            }
        }

        // print bottom
        print!("  ");
        for _ in 0..self.width * 2 + self.width + 1 {
            print!("#");
        }
        println!();

        // counts
        let all = Stride::new(&self.cells);
        let mut col_stides = all.substrides(self.width);

        print!("   ");
        for ix in 0..self.width {
            let col_x = col_stides.next().unwrap();
            let count = col_x
                .iter()
                .filter(|&&it| it.state == CellState::Flooded)
                .count();

            let col_remainder = self.col_hints[ix] - isize::try_from(count).expect("");
            print!("{}  ", { col_remainder });
        }
        println!();
        //
        if print_index {
            print!("  _");
            for _ in 0..self.width * 3 {
                print!("_");
            }
            println!();
            print!("   ");
            for ix in 0..self.width {
                print!("{}  ", ix);
            }
            println!();
        }
    }

    fn solve(&mut self) {
        use std::collections::HashMap;
        loop {
            let mut updated = false;
            // look for n_row_part > remaining -> invalidate
            for iy in (0..self.height).rev() {
                let mut map_sizes = HashMap::new(); // partitan : size
                let mut map_states = HashMap::new(); // partitian : state
                let mut map_totals = HashMap::new(); // state: count

                for ix in 0..self.width {
                    let cell = self.cell_at(ix, iy);
                    let count = map_sizes.entry(cell.partition).or_insert(0);
                    *count += 1;

                    map_states.entry(cell.partition).or_insert(cell.state);
                }
                for (part, state) in &map_states {
                    let total = map_totals.entry(*state).or_insert(0);
                    *total += map_sizes[part];
                }

                // println!("{} filled: {:?}", iy, map_totals);
                // println!("{} filled: {:?}", iy, map_states);
                // println!("{} counts: {:?}", iy, map_sizes);

                let remainder =
                    self.row_hints[iy] - map_totals.get(&CellState::Flooded).unwrap_or(&0);
                for ix in 0..self.width {
                    let cell_ix = self.cell_at(ix, iy);
                    if cell_ix.state != CellState::Empty {
                        continue;
                    };

                    // !!!
                    if map_sizes[&cell_ix.partition] > remainder {
                        println!("Invalidate {}, {}", ix, iy);
                        self.invalidate(ix, iy);
                        updated = true;
                    }
                }
            }

            if !updated {
                break;
            }
        }
    }
}

fn game() {
    // let board = Board::make(3, 3);
    // board.print0();

    let mut board = Board::make_b0();
    let board_solved = Board::make_b0_solved();
    board.print();

    //
    // board.flood(0, 0);
    // board.invalidate(0, 5);
    board.solve();
    println!("\n");
    board.print();

    // println!("\n");
    // board_solved.print();
}

fn idk() {
    // use std::convert::TryInto;
    // let a: usize = 0;
    // let b:isize = (a).try_into().unwrap()-1;
    // println!("{}", b);
}

fn main() {
    game();
    // idk();
}
