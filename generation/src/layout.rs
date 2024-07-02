use std::collections::VecDeque;

use rand::Rng;

use super::{constants::_SIZE, metadata::MetaData};

const AS_USIZE: usize = _SIZE as usize;

#[derive(Clone, Copy, Default)]
pub(super) struct Cell {
    pub x: u8,
    pub y: u8,
    pub of_type: u8,
    pub type_representation: Option<char>,
}

pub(super) struct Layout {
    pub cells: [[Cell; AS_USIZE]; AS_USIZE],
    pub entrance: Cell,
    pub exit: Cell,
}

/*
 0 = empty
 1 = entrance
 2 = exit
 3 = player
 4 = liquid
 5 = wall
*/
impl Layout {
    pub(super) fn new() -> Self {
        let mut cells = [[Cell::default(); AS_USIZE]; AS_USIZE];
        let entrance_exit = Self::determine_start_end();
        let entrance = entrance_exit.0;
        let exit = entrance_exit.1;
        for row in 0.._SIZE {
            for column in 0.._SIZE {
                let mut of_type = 0;
                let mut type_representation = None;
                let row_as_u8 = row as u8;
                let column_as_u8 = column as u8;

                if row == _SIZE - 1 as u8 && column == entrance.x {
                    of_type = entrance.of_type;
                    type_representation = entrance.type_representation;
                }

                if row == 0 as u8 && column == exit.x {
                    of_type = exit.of_type;
                    type_representation = exit.type_representation;
                }

                cells[row as usize][column as usize] = Cell {
                    x: column_as_u8,
                    y: row_as_u8,
                    of_type: of_type,
                    type_representation: type_representation,
                };
            }
        }
        Layout {
            cells: cells,
            entrance: entrance,
            exit: exit,
        }
    }

    fn determine_start_end() -> (Cell, Cell) {
        let entrance = Cell {
            x: Self::get_single_coordinate() as u8,
            y: _SIZE - 1,
            of_type: 1,
            type_representation: Some('E'),
        };
        let exit = Cell {
            x: Self::get_single_coordinate() as u8,
            y: 0,
            of_type: 2,
            type_representation: Some('X'),
        };

        return (entrance, exit);
    }

    pub(super) fn populate(&mut self, meta_data: &mut MetaData) {
        let mut current_liquid_block: Vec<Cell> = Vec::new();
        current_liquid_block.push(self.spawn_liquid_source(meta_data.liquid_block_type));

        meta_data.current_liquid_blocks = current_liquid_block.len() as u8;

        let mut current_wall_block: Vec<Cell> = Vec::new();
    }

    fn spawn_liquid_source(&mut self, liquid_block_type: Option<char>) -> Cell {
        loop {
            let row = Self::get_single_coordinate();
            let column = Self::get_single_coordinate();

            if self.is_blocked(column, row) {
                continue;
            }

            let cell = Cell {
                x: column,
                y: row,
                of_type: 4,
                type_representation: liquid_block_type,
            };

            self.cells[row as usize][column as usize] = cell;

            return cell;
        }
    }

    fn fill_liquid(&self, cells: &mut Vec<Cell>, meta_data: &mut MetaData) {
        let first_block = cells[0];
    }

    fn is_blocked(&self, x: u8, y: u8) -> bool {
        if self.entrance.x == x && self.entrance.y == y {
            return true;
        }

        if self.exit.x == x && self.exit.y == y {
            return true;
        }

        false
    }

    fn get_single_coordinate() -> u8 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(0.._SIZE - 1);
    }

    fn is_valid_cell(&self, visited: &Vec<Vec<bool>>, x: i8, y: i8) -> bool {
        let border: i8 = self.cells.len() as i8 - 1;
        // Make sure we don't go below 0
        if (x < 0) || (y < 0) {
            return false;
        }

        // Or above the maximum size of the vec
        if (x > border) || (y > border) {
            return false;
        }

        // If it is already visited, we don't have to touch it again
        if visited[y as usize][x as usize] {
            return false;
        }

        // See if the adjacent cell is a traversable cell
        let is_type = self.cells[y as usize][x as usize].of_type;
        if is_type > 2 {
            return false;
        }

        true
    }

    pub(super) fn bfs(&self) -> bool {
        // left, right, up, down
        let directions: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let start = &self.entrance;

        // According to the all knowning Chat Gippity it is preferable to use VecDeque in these situations, because it
        // optimizes push/pop_front and whatnot.
        let mut queue: VecDeque<(u8, u8)> = VecDeque::new();
        // We will mark the nodes we have visited, so we don't check those again.
        let mut visited = vec![vec![false; AS_USIZE]; AS_USIZE];
        // Add the starting node of the map to the vec.
        queue.push_front((start.x, start.y));
        // Mark the starting cell as visited.
        visited[start.x as usize][start.y as usize] = true;
        // While there is still a tuple of coordinates in the queue, process it.
        while let Some(cell) = queue.pop_front() {
            // We're gonna check if we can go up, right, down, left. If possible, we add it to the queue.
            let ix = cell.0 as i8;
            let iy = cell.1 as i8;

            for &(x, y) in &directions {
                let new_x = ix + x;
                let new_y = iy + y;

                if self.is_valid_cell(&visited, new_x, new_y) {
                    let next = ((ix + x) as u8, (iy + y) as u8);
                    visited[new_y as usize][new_x as usize] = true;
                    queue.push_back(next);
                }
            }
        }

        for row in &visited {
            println!("{:?}", row);
        }
        // lastly we check if the exit has been traversed.
        visited[self.exit.y as usize][self.exit.x as usize]
    }
}
