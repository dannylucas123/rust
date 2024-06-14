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
        layout_instance.populate(&mut metadata_instance);
        Map {
            meta_data: metadata_instance,
            name: location.as_str().to_string(),
            layout: layout_instance,
        }
    }

    pub fn print_layout(&self) {
        self.layout.bfs();
        for column in &self.layout.cells {
            for cell in column {
                let letter: String = match cell.of_type {
                    0 => " ".to_string(),
                    _ => match cell.type_representation {
                        Some(c) => c.to_string(),
                        None => " ".to_string(),
                    },
                };
                print!("[ {} ]", letter);
            }
            println!();
        }
    }
}
