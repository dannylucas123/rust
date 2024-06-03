use super::layout::Cell;
use super::metadata::MetaData;

pub const PLAINS: MetaData = MetaData {
    liquid_block_type: 'W',
    wall_block_type: 'T',
    max_liquid_blocks: 6,
    max_wall_blocks: 10,
    current_liquid_blocks: 0,
    current_wall_blocks: 0,
    entrance_cell: Cell {
        x: 0,
        y: 0,
        of_type: Some(0),
    },
    exit_cell: Cell {
        x: 1,
        y: 1,
        of_type: Some(0),
    },
};
