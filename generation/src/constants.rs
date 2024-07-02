use super::layout::Cell;
use super::metadata::MetaData;

pub const _SIZE: u8 = 7;
pub(super) const PLAINS: MetaData = MetaData {
    liquid_block_type: Some('W'),
    wall_block_type: Some('T'),
    max_liquid_blocks: 6,
    max_wall_blocks: 10,
    current_liquid_blocks: 0,
    current_wall_blocks: 0,
    entrance_cell: Cell {
        x: 0,
        y: 0,
        of_type: 1,
        type_representation: Some('E'),
    },
    exit_cell: Cell {
        x: 1,
        y: 1,
        of_type: 2,
        type_representation: Some('X'),
    },
};
