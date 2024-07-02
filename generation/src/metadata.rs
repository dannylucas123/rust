use super::layout::Cell;

#[derive(Clone, Copy)]
pub(super) struct MetaData {
    pub(super) liquid_block_type: Option<char>,
    pub(super) wall_block_type: Option<char>,
    pub(super) max_liquid_blocks: u8,
    pub(super) max_wall_blocks: u8,
    pub(super) current_liquid_blocks: u8,
    pub(super) current_wall_blocks: u8,
    pub(super) entrance_cell: Cell,
    pub(super) exit_cell: Cell,
}

impl MetaData {
    pub(super) fn new_from_const(meta: MetaData) -> Self {
        meta
    }

    pub(super) fn set_current_liquid_blocks(&mut self, value: u8) {
        self.current_liquid_blocks = value;
    }

    pub(super) fn set_current_wall_blocks(&mut self, value: u8) {
        self.current_wall_blocks = value;
    }
}
