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
                    x: row_as_u8,
                    y: column_as_u8,
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
        let mut rng = rand::thread_rng();

        let entrance = Cell {
            x: rng.gen_range(0..=_SIZE) as u8,
            y: _SIZE - 1,
            of_type: 1,
            type_representation: Some('E'),
        };
        let exit = Cell {
            x: rng.gen_range(0..=_SIZE) as u8,
            y: 0,
            of_type: 2,
            type_representation: Some('X'),
        };

        return (entrance, exit);
    }

    pub(super) fn populate(&mut self, meta_data: &mut MetaData) {
        let mut current_liquid_block: Vec<Cell> = Vec::new();
        let mut current_wall_block: Vec<Cell> = Vec::new();
        let result = self.spawn_liquid();
    }

    fn spawn_liquid(&self) -> (u8, u8) {
        let mut rng = rand::thread_rng();

        loop {
            let source_block_x = rng.gen_range(0.._SIZE);
            let source_block_y = rng.gen_range(0.._SIZE);

            if self.is_blocked(source_block_x, source_block_y) {
                continue;
            }

            return (source_block_x, source_block_y);
        }
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
}
