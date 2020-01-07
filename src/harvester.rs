use crate::{creep_actions, job_manager};
use log::*;
use screeps::{find, prelude::*, ObjectId, ResourceType, ReturnCode};

pub fn run_harvester(
    creep: screeps::objects::Creep,
    available_jobs: &mut job_manager::AvailableJobs,
) {
    let name = creep.name();
    debug!("running creep {}", name);

    if creep.spawning() {
        return;
    }

    // first check: did we fill up with energy or did we run out? if so, update what we're doing

    if creep.memory().bool("harvesting") {
        if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
            creep.memory().set("harvesting", false);
            creep.memory().del("source");
        } else if creep.memory().string("source").unwrap().is_none() {
            match available_jobs
                .pop_harvest_job_for_room(creep.room().unwrap().name())
                .or_else(|| available_jobs.pop_harvest_job_any())
            {
                Some(target_source) => {
                    creep.memory().set("source", target_source.id().to_string());
                    creep.memory().set("harvesting", true);
                }
                None => (),
            }
        }
    }

    // second check: if we're harvesting, go do that. if we're using energy, go do that

    if creep.memory().bool("harvesting") && creep.memory().string("source").unwrap().is_some() {
        let source_id_raw = creep.memory().string("source");
        let source_id: ObjectId<screeps::objects::Source> =
            source_id_raw.unwrap().unwrap().parse().unwrap();
        let source_opt = screeps::game::get_object_typed(source_id).unwrap();

        match source_opt {
            Some(source) => {
                let near_to_result = creep.pos().is_near_to(&source);
                if near_to_result {
                    let r = creep.harvest(&source);
                    if r != ReturnCode::Ok {
                        debug!("couldn't harvest: {:?}", r)
                    }
                } else {
                    creep.move_to(&source);
                }
            }
            None => {
                creep.memory().del("source");
            }
        }
    } else {
        if creep.energy() == 0 {
            creep.memory().set("harvesting", true);
        } else {
            spend_energy(creep)
        }
    }
}

fn spend_energy(creep: screeps::objects::Creep) {
    if creep_actions::repair_local_road(&creep) {
        return;
    }

    let mem = screeps::memory::root();
    let home_room_name_str = mem
        .string("home_room")
        .unwrap_or_else(|_| log_panic("unable to load home_room"))
        .unwrap_or_else(|| log_panic("home_room value was None"));
    let home_room_name = screeps::local::RoomName::new(&home_room_name_str).unwrap();
    let home_room = screeps::game::rooms::get(home_room_name).unwrap();

    let construction_sites = creep.room().unwrap().find(find::MY_CONSTRUCTION_SITES);

    let structures = home_room.find(find::STRUCTURES);
    let mut towers: std::vec::Vec<screeps::objects::Structure> = vec![];
    let mut extensions: std::vec::Vec<screeps::objects::Structure> = vec![];
    for my_structure in structures {
        match my_structure {
            screeps::Structure::Tower(ref my_tower) => {
                if my_tower.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                    towers.push(my_structure);
                }
            }
            screeps::Structure::Extension(ref my_extension) => {
                if my_extension.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                    extensions.push(my_structure);
                }
            }
            _ => (),
        };
    }

    let is_small = creep.body().len() <= 5;

    if is_small && towers.len() > 0 {
        creep_actions::fill(creep, &towers[0]);
    } else if is_small && extensions.len() > 0 {
        creep_actions::fill(creep, &extensions[0]);
    } else if home_room.storage().unwrap().energy() < 2000 {
        let the_storage = home_room.storage().unwrap();
        let as_structure = screeps::objects::Structure::Storage(the_storage);
        creep_actions::fill(creep, &as_structure);
    } else if construction_sites.len() > 0 {
        creep_actions::build(creep, &construction_sites[0]);
    } else if home_room.storage().unwrap().energy() < 25000 {
        let the_storage = home_room.storage().unwrap();
        let as_structure = screeps::objects::Structure::Storage(the_storage);
        creep_actions::fill(creep, &as_structure);
    } else if home_room.storage().unwrap().energy() < 950000 && !is_small {
        let the_storage = home_room.storage().unwrap();
        let as_structure = screeps::objects::Structure::Storage(the_storage);
        creep_actions::fill(creep, &as_structure);
    } else {
        creep_actions::upgrade_controller(creep, &home_room.controller().unwrap());
    };
}

fn log_panic<T>(message: &str) -> T {
    error!("unable to unwrap value: {}", message);
    panic!()
}
