use super::constants::PLAINS;
use super::layout::Layout;
use super::location::Location;
use super::metadata::MetaData;

pub struct Map {
    pub name: String,
    pub meta_data: MetaData,
    pub layout: Layout,
}

impl Map {
    pub fn new(location: Location) -> Self {
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
            name: location.as_str().to_string(),
            layout: layout_instance,
        }
    }

    pub fn print_layout(&self) {
        for row in &self.layout.cells {
            for cell in row {
                let letter = match cell.of_type {
                    Some(0) => " ",
                    Some(1) => "E",
                    Some(2) => "X",
                    _ => " ",
                };
                print!("[ {} ]", letter);
            }
            println!();
        }
    }
}
