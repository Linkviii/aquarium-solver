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
}

// Should probably change to this

// struct Cell {
//     state: CellState,
//     partition: isize
// }

// impl Cell {
//     fn rep(&self) -> char {
//         self.rep()
//     }
// }

struct Board {
    // Visual properties of the board
    width: usize,
    height: usize,
    // width x height
    cells: Vec<CellState>,
    // width-1 x height
    walls: Vec<WallState>,
    // width x height-1
    floors: Vec<FloorState>,
    // height
    row_hints: Vec<isize>,
    // width
    col_hints: Vec<isize>,

    // Meta properties of the board
    partitions: Vec<isize>,
    // TODO ?
    // First draft described the board by its walls and floors,
    // but that is redundant information.
    // Would probably be best to remove floors and walls.
}

impl Board {
    fn cell_at(&self, ix: usize, iy: usize) -> CellState {
        assert!(ix < self.width && iy < self.height);
        let row_offset = iy * self.width;
        self.cells[row_offset + ix]
    }

    fn set_cell_at(&mut self, ix: usize, iy: usize, state: CellState) {
        assert!(ix < self.width && iy < self.height);
        let row_offset = iy * self.width;
        self.cells[row_offset + ix] = state;
    }

    fn partition_at(&self, ix: usize, iy: usize) -> isize {
        assert!(ix < self.width && iy < self.height);
        let row_offset = iy * self.width;
        self.partitions[row_offset + ix]
    }

    fn wall_at(&self, ix: usize, iy: usize) -> WallState {
        assert!(ix < self.width - 1 && iy < self.height);
        let row_offset = iy * (self.width - 1);
        self.walls[row_offset + ix]
    }
    //
    fn floor_at(&self, ix: usize, iy: usize) -> FloorState {
        assert!(ix < self.width && iy < self.height - 1);
        let row_offset = iy * self.width;
        self.floors[row_offset + ix]
    }

    /// Recursively set all adjacent cells not seperated by a wall or floor to be in this partition
    fn partion_cell(&mut self, ix: usize, iy: usize, partition: isize) -> bool {
        let cell_row_offset = iy * self.width;

        // already partioned
        if self.partitions[cell_row_offset + ix] != -1 {
            return false;
        }

        //
        self.partitions[cell_row_offset + ix] = partition;

        if iy != 0 && self.floor_at(ix, iy - 1) != FloorState::Bott {
            self.partion_cell(ix, iy - 1, partition);
        }
        if ix != 0 && self.wall_at(ix - 1, iy) != WallState::Wall {
            self.partion_cell(ix - 1, iy, partition);
        }
        if iy + 1 != self.height && self.floor_at(ix, iy) != FloorState::Bott {
            self.partion_cell(ix, iy + 1, partition);
        }
        if ix + 1 != self.width && self.wall_at(ix, iy) != WallState::Wall {
            self.partion_cell(ix + 1, iy, partition);
        }

        true
    }

    /// Derive the partitions of the board based on the walls and floors
    fn init_partitions(&mut self) {
        let mut partion = 0;
        for iy in 0..self.height {
            for ix in 0..self.width {
                if self.partion_cell(ix, iy, partion) {
                    partion += 1;
                }
            }
        }
    }

    // TODO
    fn make(width: usize, height: usize) -> Board {
        let mut board = Board {
            width,
            height,
            cells: Vec::new(),
            walls: Vec::new(),
            floors: Vec::new(),
            row_hints: vec![0; height],
            col_hints: vec![0; width],
            partitions: vec![-1; width * height],
        };
        board.init_partitions();
        board
    }

    fn make_b0() -> Board {
        // 6x6 Easy ID: 3,095,209 https://www.puzzle-aquarium.com/specfic.php

        let walls = {
            use WallState::*;
            vec![
                None, None, None, Wall, None, //
                None, Wall, None, Wall, None, //
                Wall, Wall, Wall, Wall, Wall, //
                None, None, Wall, Wall, Wall, //
                None, None, None, None, Wall, //
                None, Wall, None, None, None,
            ]
        };
        let floors = {
            use FloorState::*;
            vec![
                None, None, Bott, Bott, None, None, //
                Bott, None, Bott, None, Bott, Bott, //
                None, Bott, None, None, None, None, //
                None, None, None, Bott, Bott, None, //
                None, None, Bott, Bott, Bott, None,
            ]
        };

        let mut board = Board {
            width: 6,
            height: 6,
            cells: vec![CellState::Empty; 6 * 6],
            walls,
            floors,
            row_hints: vec![2, 4, 3, 2, 1, 4],
            col_hints: vec![1, 2, 1, 3, 5, 4],
            partitions: vec![-1; 6 * 6],
        };
        board.init_partitions();
        board
    }

    fn make_b0_solved() -> Board {
        let mut board = Board::make_b0();
        use CellState::*;

        let cells = vec![
            Invalid, Invalid, Invalid, Invalid, Flooded, Flooded, //
            Flooded, Flooded, Invalid, Invalid, Flooded, Flooded, //
            Invalid, Flooded, Invalid, Flooded, Flooded, Invalid, //
            Invalid, Invalid, Invalid, Flooded, Flooded, Invalid, //
            Invalid, Invalid, Invalid, Invalid, Invalid, Flooded, //
            Invalid, Invalid, Flooded, Flooded, Flooded, Flooded,
        ];

        board.cells = cells;

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
        let print_index = true;
        let print_partitions = false;

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
            let row_walls = {
                let row_offset = iy * (self.width - 1);
                &self.walls[row_offset..row_offset + self.width - 1]
            };

            // Print cells and walls
            for ix in 0..self.width {
                if !print_partitions {
                    print!("{} ", row_cells[ix].rep());
                } else {
                    print!(
                        "{}{}",
                        row_cells[ix].rep(),
                        self.partitions[ix + iy * self.width]
                    );
                }

                if ix + 1 != self.width {
                    print!("{}", row_walls[ix].rep());
                }
            }

            let n_row = row_cells
                .iter()
                .filter(|&&state| state == CellState::Flooded)
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
                let row_floor = {
                    let row_offset = iy * self.width;
                    &self.floors[row_offset..row_offset + self.width]
                };

                print!("  #");
                for (ix, it) in row_floor.iter().enumerate() {
                    let rep = it.rep();

                    // Up, Left (this), Right, Down
                    let walled_neighbors = [
                        if ix + 1 == self.width {
                            true
                        } else {
                            self.wall_at(ix, iy) == WallState::Wall
                        },
                        *it == FloorState::Bott,
                        if ix + 1 == self.width {
                            true
                        } else {
                            self.floor_at(ix + 1, iy) == FloorState::Bott
                        },
                        if iy + 1 == self.height || ix + 1 == self.width {
                            true
                        } else {
                            self.wall_at(ix, iy + 1) == WallState::Wall
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
            let count = col_x.iter().filter(|&&it| it == CellState::Flooded).count();

            let col_remainder = self.col_hints[ix] - isize::try_from(count).expect("");
            print!("{}  ", { col_remainder });

            // index
            // print!(" |  {}", iy);
        }
        println!();
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
                    let part = self.partition_at(ix, iy);
                    let count = map_sizes.entry(part).or_insert(0);
                    *count += 1;

                    // map_states.entry(part).or_insert(row_cells[ix]);
                    map_states.entry(part).or_insert(self.cell_at(ix, iy));
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
                    // let cell_ix = row_cells[ix];
                    let cell_ix = self.cell_at(ix, iy);
                    if cell_ix != CellState::Empty {
                        continue;
                    };
                    // let part_ix = row_partitions[ix];
                    let part_ix = self.partition_at(ix, iy);

                    // !!!
                    if map_sizes[&part_ix] > remainder {
                        println!("Invalidate {}, {}", ix, iy);
                        self.invalidate(ix, iy);
                        // return; // XXX
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
