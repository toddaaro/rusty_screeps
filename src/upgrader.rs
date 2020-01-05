use crate::creep_actions;
use log::*;
use screeps::{prelude::*, ResourceType, ReturnCode};

pub fn run_upgrader(creep: screeps::objects::Creep) {
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
            creep_actions::upgrade_controller(creep, &home_room.controller().unwrap());
        }
    }
}
