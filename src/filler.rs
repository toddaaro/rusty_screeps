use log::*;
use screeps::{find, prelude::*, ResourceType, ReturnCode};
use std::cmp::min;

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
                fill(creep, &towers[0]);
            } else if extensions.len() > 0 {
                fill(creep, &extensions[0]);
            }
        }
    }
}

fn fill(creep: screeps::objects::Creep, fill_target: &screeps::objects::Structure) {
    let transferable = fill_target.as_transferable().unwrap();
    let has_store = fill_target.as_has_store().unwrap();

    let empty_space = has_store.store_free_capacity(Some(ResourceType::Energy));
    let creep_energy = creep.energy();
    let amount = min(creep_energy, empty_space);

    let r = creep.transfer_amount(transferable, ResourceType::Energy, amount);
    if r == ReturnCode::NotInRange {
        creep.move_to(fill_target);
    } else if r == ReturnCode::Full {
        creep.memory().del("fill_target");
    } else if r != ReturnCode::Ok {
        warn!("couldn't transfer: {:?}", r);
    }
}
