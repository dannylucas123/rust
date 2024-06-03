enum Location {
    Plains,
    Town,
    Forest,
    Dungeon,
}

const PLAINS: MetaData = MetaData {
    liquid_block_type: 'W',
    wall_block_type: 'T',
    max_liquid_blocks: 6,
    max_wall_blocks: 10,
    current_liquid_blocks: 0,
    current_wall_blocks: 0,
    entrance_cell: Cell { x: 0, y: 0 },
    exit_cell: Cell { x: 1, y: 1 },
};

#[derive(Clone, Copy)]
struct MetaData {
    liquid_block_type: char,
    wall_block_type: char,
    max_liquid_blocks: u8,
    max_wall_blocks: u8,
    current_liquid_blocks: u8,
    current_wall_blocks: u8,
    entrance_cell: Cell,
    exit_cell: Cell,
}

impl MetaData {
    fn new_from_const(meta: MetaData) -> Self {
        meta
    }

    fn set_current_liquid_blocks(&mut self, value: u8) {
        self.current_liquid_blocks = value;
    }

    fn set_current_wall_blocks(&mut self, value: u8) {
        self.current_wall_blocks = value;
    }
}

#[derive(Clone, Copy, Default)]
struct Cell {
    x: u8,
    y: u8,
}

struct Layout {
    cells: [[Cell; 7]; 7],
}

impl Layout {
    fn new() -> Self {
        Layout {
            cells: [[Cell::default(); 7]; 7],
        }
    }
}

struct Map {
    name: String,
    meta_data: MetaData,
    layout: Layout,
}

impl Map {
    fn new(location: Location) -> Self {
        let metadata: MetaData = match location {
            Location::Dungeon => PLAINS,
            Location::Plains => PLAINS,
            Location::Town => PLAINS,
            Location::Forest => PLAINS,
        };
        let mut metadata_instance = MetaData::new_from_const(metadata);
        let mut layout_instance = Layout::new();
        Map {
            meta_data: metadata_instance,
            name: "Plains".to_string(),
            layout: layout_instance,
        }
    }

    fn print_layout(&self) {
        let mut c = 1;
        for row in &self.layout.cells {
            for cell in row {
                if c < 3 {
                    print!("[ {} ]", c);
                } else {
                    print!("[ {} ]", " ");
                }
            }
            println!();
            c += 1;
        }
    }
}

fn main() {
    let map: Map = Map::new(Location::Plains);
    println!("Printing the {} map.", map.name);
    map.print_layout();
}
