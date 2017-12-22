use std::collections::HashMap;

use constants;


pub struct ResourceSlots {
    types: &'static [constants::ResourceType],
    size: u32,
}


pub trait Room {
    fn get_name(&self) -> &str;

    fn is_unique(&self) -> bool {
        true
    }

    fn get_gnome_slots(&self) -> u32 {
        0
    }

    fn get_score_points(&self) -> u32 {
        0
    }

    fn get_slots(&self) -> ResourceSlots {
        ResourceSlots {
            types: constants::TRIBAL_ANIMALS,
            size: 0,
        }
    }

    fn get_price(&self) -> HashMap<String, u32>;
}

pub fn get_from_string(string: &str) -> Result<&Room, String> {
    let mut dwellings: HashMap<&str, &Room> = HashMap::new();

    dwellings.insert(EntryLevelDwelling {}.get_name(), &EntryLevelDwelling {});
    dwellings.insert(Dwelling {}.get_name(), &Dwelling {});
    dwellings.insert(SimpleDwelling1 {}.get_name(), &SimpleDwelling1 {});
    dwellings.insert(SimpleDwelling2 {}.get_name(), &SimpleDwelling2 {});
    dwellings.insert(CoupleDwelling {}.get_name(), &CoupleDwelling {});
    dwellings.insert(AdditionalDwelling {}.get_name(), &AdditionalDwelling {});

    match dwellings.get(string) {
        Some(x) => Ok(*x),
        None => Err(format!("No room for {} found", string)),
    }
}



pub struct EntryLevelDwelling {}

impl Room for EntryLevelDwelling {
    fn get_name(&self) -> &str {
        "entry_level_dwelling"
    }

    fn get_gnome_slots(&self) -> u32 {
        2
    }

    fn get_slots(&self) -> ResourceSlots {
        ResourceSlots {
            types: constants::TRIBAL_ANIMALS,
            size: 2,
        }
    }

    fn get_price(&self) -> HashMap<String, u32> {
        hash_map! {
            constants::ResourceType::Gold.str_key() => 0
        }
    }
}


pub struct Dwelling {}

impl Room for Dwelling {
    fn get_name(&self) -> &str {
        "dwelling"
    }

    fn get_gnome_slots(&self) -> u32 {
        1
    }

    fn is_unique(&self) -> bool {
        false
    }

    fn get_price(&self) -> HashMap<String, u32> {
        hash_map! {
            constants::ResourceType::Wood.str_key() => 4,
            constants::ResourceType::Stone.str_key() => 3
        }
    }
}

pub struct SimpleDwelling1 {}

impl Room for SimpleDwelling1 {
    fn get_name(&self) -> &str {
        "simple_dwelling1"
    }

    fn get_gnome_slots(&self) -> u32 {
        1
    }

    fn get_price(&self) -> HashMap<String, u32> {
        hash_map! {
            constants::ResourceType::Wood.str_key() => 4,
            constants::ResourceType::Stone.str_key() => 2
        }
    }
}

pub struct SimpleDwelling2 {}

impl Room for SimpleDwelling2 {
    fn get_name(&self) -> &str {
        "simple_dwelling2"
    }

    fn get_gnome_slots(&self) -> u32 {
        1
    }

    fn get_price(&self) -> HashMap<String, u32> {
        hash_map! {
            constants::ResourceType::Wood.str_key() => 3,
            constants::ResourceType::Stone.str_key() => 3
        }
    }
}

pub struct MixedDwelling {}

impl Room for MixedDwelling {
    fn get_name(&self) -> &str {
        "mixed_dwelling"
    }

    fn get_gnome_slots(&self) -> u32 {
        1
    }

    fn get_slots(&self) -> ResourceSlots {
        ResourceSlots {
            types: constants::TRIBAL_ANIMALS,
            size: 2,
        }
    }

    fn get_price(&self) -> HashMap<String, u32> {
        hash_map! {
            constants::ResourceType::Wood.str_key() => 3,
            constants::ResourceType::Stone.str_key() => 3
        }
    }
}

pub struct CoupleDwelling {}

impl Room for CoupleDwelling {
    fn get_name(&self) -> &str {
        "couple_dwelling"
    }

    fn get_gnome_slots(&self) -> u32 {
        2
    }

    fn get_price(&self) -> HashMap<String, u32> {
        hash_map! {
            constants::ResourceType::Wood.str_key() => 8,
            constants::ResourceType::Stone.str_key() => 6
        }
    }
}

pub struct AdditionalDwelling {}

impl Room for AdditionalDwelling {
    fn get_name(&self) -> &str {
        "additional_dwelling"
    }

    fn get_gnome_slots(&self) -> u32 {
        1
    }

    fn get_price(&self) -> HashMap<String, u32> {
        hash_map! {
            constants::ResourceType::Wood.str_key() => 8,
            constants::ResourceType::Stone.str_key() => 6
        }
    }
}