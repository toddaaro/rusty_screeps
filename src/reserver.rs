use crate::creep_actions;
use crate::job_manager;
use log::*;
use screeps::prelude::*;

pub fn run_reserver(creep: screeps::objects::Creep) {
    debug!("running reserver {:?}", creep.id());

    if creep.memory().string("reserve_target").unwrap().is_none() {
        let available_jobs = job_manager::build_job_set();
        let reserve_target = &available_jobs.reserve_jobs[0];
        creep.memory().set("reserve_target", reserve_target);
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
                creep_actions::reserve(creep, &reserve_target);
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
