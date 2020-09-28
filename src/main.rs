// TODO
#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

use std::convert::TryFrom;
use std::convert::TryInto;
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
    fn rep(&self, show_partition: bool) -> String {
        if !show_partition {
            format!("{}  ", self.state.rep())
        } else {
            format!("{}{:>2}", self.state.rep(), self.partition)
        }
    }

    fn rep_width() -> usize {
        {
            // Todo: Learn how to check this at compile time
            // let null_cell = Cell {
            //     state: CellState::Empty,
            //     partition: 9,
            // };
            // assert_eq!(
            //     null_cell.rep(false).chars().count(),
            //     null_cell.rep(true).chars().count(),
            // );
        }
        3
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

        let cell_width = Cell::rep_width();
        let wall_width = 1;

        let board_width = wall_width + (cell_width + wall_width) * self.width;
        let board_bounds = "#".repeat(board_width);

        let left_margin = "   ";

        let right_clue_width = 4;
        let right_clue_space = " ".repeat(right_clue_width);

        // Todo: Use format width for numbers

        // print top
        // '   N0  N1 N3'
        print!("{} ", left_margin);
        for hint in &self.col_hints {
            print!("{:>2}  ", hint);
        }
        println!();

        // '  #########'
        print!("{}{}", left_margin, board_bounds);
        println!();

        //
        for iy in 0..self.height {
            // Cell row: 'N # C0 W0 C1 W1 C2 # M' ? ' |  I'
            // i.e. '3 #   |   #X  #*  # 2'
            // i.e. '3 #* 0|* 0#X 1#  2# 1 | 1'

            // Left Margin: 'N #'
            print!("{:>2} #", self.row_hints[iy]);
            //
            let row_cells = {
                let row_offset = iy * self.width;
                &self.cells[row_offset..row_offset + self.width]
            };

            let row_walls: Vec<_> = (0..self.width - 1)
                .map(|ix| WallState::rep_bool(self.wall_at(ix, iy)))
                .collect();

            // Cells and walls: 'C0 W0 C1 W1 C2'
            for ix in 0..self.width {
                print!("{}", row_cells[ix].rep(print_partitions));

                if ix + 1 != self.width {
                    print!("{}", row_walls[ix]);
                }
            }

            // Close row and remainder: '# M'
            let n_row = row_cells
                .iter()
                .filter(|&&cell| cell.state == CellState::Flooded)
                .count();
            let row_remainder = self.row_hints[iy] - isize::try_from(n_row).unwrap();
            print!("# {:>2}", row_remainder);

            // Row index: ' | I'
            if print_index {
                print!(" |  {:>2}", iy);
            }

            println!();
            //
            // Floor row: '  # F0 J0 F1 J1 #' ? '   |'
            // i.e. '  #---+---#####---#'
            // i.e. '  #---+---#####---#   |'
            if iy + 1 != self.height {
                let row_floor: Vec<_> = (0..self.width).map(|ix| self.floor_at(ix, iy)).collect();

                // Left margin: '  #'
                print!("{}#", left_margin);
                for (ix, it) in row_floor.iter().enumerate() {
                    let rep = FloorState::rep_bool(*it);
                    let rep: String = std::iter::repeat(rep).take(cell_width).collect();

                    // Up, Left (this), Right, Down
                    let junction_neighbors = [
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
                    let count = junction_neighbors.iter().filter(|&&x| x).count();
                    // `if any` would have been just as fine I guess
                    let junction = if count >= 2 { '#' } else { '+' };

                    // Cell floor and junction: 'F_ix J_ix'
                    print!("{}{}", rep, junction);
                }
                if print_index {
                    // '   |'
                    print!("{}|", right_clue_space);
                }
                println!();
            }
        }

        // Print bottom: '  #########' ? '  |'
        print!("{}{}", left_margin, board_bounds);
        if print_index {
            print!("{}|", right_clue_space);
        }
        println!();

        // Counts: '     M0 M1 M3' ? '   |'
        let all_cols = Stride::new(&self.cells);
        let mut col_stides = all_cols.substrides(self.width);

        print!("{} ", left_margin);
        for ix in 0..self.width {
            let col_x = col_stides.next().unwrap();
            let count = col_x
                .iter()
                .filter(|&&it| it.state == CellState::Flooded)
                .count();

            let col_remainder = self.col_hints[ix] - isize::try_from(count).expect("");
            print!("{:>2}  ", col_remainder);
        }

        if print_index {
            print!("{}|", right_clue_space);
        }

        println!();
        //
        //
        if print_index {
            // Axis line: '   _________|'
            print!(
                "{}{}|",
                left_margin,
                "_".repeat(board_width + right_clue_width)
            );
            println!();

            // Axis labels: '    0  1  2  3'
            print!("{} ", left_margin);
            for ix in 0..self.width {
                print!("{:>2}  ", ix);
            }
            println!();
        }
    }

    /// For the row iy, the state of each partition in the row
    fn row_partition_states(&self, iy: usize) -> HashMap<isize, CellState> {
        let mut map_states = HashMap::new();
        for ix in 0..self.width {
            let cell = self.cell_at(ix, iy);

            map_states.entry(cell.partition).or_insert(cell.state);
        }
        map_states
    }

    fn solve(&mut self) {
        let row_partitions: Vec<_> = (0..self.height)
            .map(|iy| {
                let mut map_sizes = HashMap::new();
                for ix in 0..self.width {
                    let cell = self.cell_at(ix, iy);
                    let count = map_sizes.entry(cell.partition).or_insert(0);
                    *count += 1;
                }
                map_sizes
            })
            .collect();

        // For the row iy, The number of cells in each state
        let row_state_counts =
            |map_sizes: &HashMap<isize, isize>, map_states: &HashMap<isize, CellState>| {
                let mut map_totals = HashMap::new();
                for (part, state) in map_states.iter() {
                    let total = map_totals.entry(*state).or_insert(0);
                    *total += map_sizes[part];
                }
                map_totals
            };

        loop {
            let mut updated = false;
            // look for n_row_part > remaining => invalidate
            for iy in (0..self.height).rev() {
                let map_sizes = &row_partitions[iy]; // partitan : size
                let map_states = self.row_partition_states(iy); // partitian : state
                let map_totals = row_state_counts(map_sizes, &map_states); // state: count

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

            // Look for width - n_row_part < remainder =>  flood
            for iy in 0..self.height {
                let map_sizes = &row_partitions[iy]; // partitan : size
                let map_states = self.row_partition_states(iy); // partitian : state
                let map_totals = row_state_counts(map_sizes, &map_states); // state: count

                let remainder =
                    self.row_hints[iy] - map_totals.get(&CellState::Flooded).unwrap_or(&0);

                    for ix in 0..self.width {
                        let cell_ix = self.cell_at(ix, iy);
                        if cell_ix.state != CellState::Empty {
                            continue;
                        };
                        // !!!
                        if isize::try_from(self.width).unwrap() - map_sizes[&cell_ix.partition] < remainder {
                            println!("Flood {}, {}", ix, iy);
                            self.flood(ix, iy);
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
    // let width = 3;
    // let char_a = 'a';
    // let char_pound = '#';
    // println!("|{:2$>1$}|", char_pound, width, char_a);
    let a = -1;
    let b = 1;
    let c = 10;

    // let FORMAT = "{:>2}";

    // println!(format!("|{}|", FORMAT), a);
    println!("|{:>2}|", b);
    println!("|{:>2}|", c);

    let n: usize = 11;
    // let n: usize = 3;

    let mut board = Board::make(n, n);
    for hint in board.row_hints.iter_mut() {
        *hint = n.try_into().unwrap();
    }
    for hint in board.col_hints.iter_mut() {
        *hint = n.try_into().unwrap();
    }
    board.print();
}

fn main() {
    game();
    // idk();
}
