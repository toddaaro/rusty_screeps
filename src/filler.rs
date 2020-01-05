use crate::creep_actions;
use log::*;
use screeps::{find, prelude::*, ResourceType, ReturnCode};

pub fn run_filler(creep: screeps::objects::Creep) {
    let name = creep.name();
    debug!("running creep {}", name);

    if creep.spawning() {
        return;
    }

    let mem = screeps::memory::root();
    let home_room_name_str = mem.string("home_room").unwrap().unwrap();
    let home_room_name = screeps::local::RoomName::new(&home_room_name_str).unwrap();
    let home_room = screeps::game::rooms::get(home_room_name).unwrap();
    let storage = home_room.storage().unwrap();

    if creep.memory().bool("withdrawing") {
        if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
            creep.memory().set("withdrawing", false);
        } else {
            let r = creep.withdraw_all(&storage, ResourceType::Energy);
            if r == ReturnCode::NotInRange {
                creep.move_to(&storage);
            } else if r != ReturnCode::Ok {
                warn!("couldn't withdraw: {:?}", r);
            }
        }
    } else {
        if creep.energy() == 0 {
            creep.memory().set("withdrawing", true);
        } else {
            let structures = home_room.find(find::STRUCTURES);
            let mut towers: std::vec::Vec<screeps::objects::Structure> = vec![];
            let mut extensions: std::vec::Vec<screeps::objects::Structure> = vec![];
            for my_structure in structures {
                match my_structure {
                    screeps::Structure::Tower(ref my_tower) => {
                        if my_tower.store_free_capacity(Some(ResourceType::Energy)) > 100 {
                            towers.push(my_structure);
                        }
                    }
                    screeps::Structure::Extension(ref my_extension) => {
                        if my_extension.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                            extensions.push(my_structure);
                        }
                    }
                    screeps::Structure::Spawn(ref my_spawn) => {
                        if my_spawn.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                            extensions.push(my_structure);
                        }
                    }
                    _ => (),
                };
            }
            if towers.len() > 0 {
                creep_actions::fill(creep, &towers[0]);
            } else if extensions.len() > 0 {
                creep_actions::fill(creep, &extensions[0]);
            }
        }
    }
}
