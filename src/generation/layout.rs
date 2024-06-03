use rand::Rng;

#[derive(Clone, Copy, Default)]
pub(super) struct Cell {
    pub x: u8,
    pub y: u8,
    pub of_type: Option<u8>,
}

pub(super) struct Layout {
    pub cells: [[Cell; 7]; 7],
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
        let mut cells = [[Cell::default(); 7]; 7];
        let entrance_exit = Self::determine_start_end();
        let entrance = entrance_exit.0;
        let exit = entrance_exit.1;
        for row in 0..7 {
            for column in 0..7 {
                let mut of_type = 0;
                let row_as_u8 = row as u8;
                let column_as_u8 = column as u8;

                if row == 6 as u8 && column == entrance.x {
                    of_type = 1;
                }

                if row == 0 as u8 && column == exit.x {
                    of_type = 2;
                }

                cells[row as usize][column as usize] = Cell {
                    x: row_as_u8,
                    y: column_as_u8,
                    of_type: Some(of_type),
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
            x: rng.gen_range(0..=6) as u8,
            y: 6,
            of_type: None,
        };
        let exit = Cell {
            x: rng.gen_range(0..=6) as u8,
            y: 0,
            of_type: None,
        };

        return (entrance, exit);
    }
}
