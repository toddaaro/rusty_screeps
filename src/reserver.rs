use log::*;
use screeps::{prelude::*, ReturnCode};
use std::collections::HashMap;

pub fn run_reserver(creep: screeps::objects::Creep) {
    debug!("running reserver {:?}", creep.id());

    if creep.memory().string("reserve_target").unwrap().is_none() {
        let mem = screeps::memory::root();
        let reserved_rooms: std::vec::Vec<String> = mem.arr("reserved_rooms").unwrap().unwrap();

        warn!("got targets");

        let mut worked_reservations = HashMap::new();

        for room in reserved_rooms {
            worked_reservations.entry(room).or_insert(0);
        }

        for creep in screeps::game::creeps::values() {
            let target_opt = creep.memory().string("reserve_target").unwrap();
            match target_opt {
                Some(target) => {
                    let count = worked_reservations.entry(target).or_insert(0);
                    *count += 1;
                }
                None => (),
            };
        }

        for (room_str, count) in worked_reservations {
            if count == 0 {
                creep.memory().set("reserve_target", room_str);
                break;
            }
        }
    }
    if creep.memory().string("reserve_target").unwrap().is_none() {
        warn!("tried to find a reserve target but failed, too many reservers?");
    } else {
        let room_str = creep.memory().string("reserve_target").unwrap().unwrap();
        let room_name = screeps::local::RoomName::new(&room_str).unwrap();
        let room_result = screeps::game::rooms::get(room_name);

        match room_result {
            Some(room) => {
                let reserve_target = room.controller().unwrap();
                reserve(creep, &reserve_target);
            }
            _ => {
                let exit_direction =
                    screeps::game::map::find_exit(creep.room().unwrap().name(), room_name).unwrap();
                let exit = match exit_direction {
                    screeps::constants::ExitDirection::Top => {
                        let dir = screeps::constants::find::EXIT_TOP;
                        creep.room().unwrap().find(dir)[0]
                    }
                    screeps::constants::ExitDirection::Right => {
                        let dir = screeps::constants::find::EXIT_RIGHT;
                        creep.room().unwrap().find(dir)[0]
                    }
                    screeps::constants::ExitDirection::Bottom => {
                        let dir = screeps::constants::find::EXIT_BOTTOM;
                        creep.room().unwrap().find(dir)[0]
                    }
                    screeps::constants::ExitDirection::Left => {
                        let dir = screeps::constants::find::EXIT_LEFT;
                        creep.room().unwrap().find(dir)[0]
                    }
                };
                creep.move_to(&exit);
            }
        }
    }
}

fn reserve(creep: screeps::objects::Creep, target: &screeps::objects::StructureController) {
    let r = creep.reserve_controller(target);
    if r == ReturnCode::NotInRange {
        creep.move_to(target);
    } else if r != ReturnCode::Ok {
        warn!("couldn't reserve: {:?}", r);
    }
}
