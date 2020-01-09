use crate::{creep_actions, job_manager};
use log::*;
use screeps::{find, prelude::*, ResourceType};

pub fn run_settler(
    creep: screeps::objects::Creep,
    available_jobs: &mut job_manager::AvailableJobs,
) {
    let name = creep.name();
    debug!("running creep {}", name);

    if creep.spawning() {
        return;
    }

    if creep.memory().bool("harvesting") {
        if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
            creep.memory().set("harvesting", false);
            creep.memory().del("source");
        } else if creep.memory().string("source").unwrap().is_none() {
            match available_jobs.pop_harvest_job_for_room(creep.room().unwrap().name()) {
                Some(target_source) => {
                    creep.memory().set("source", target_source.id().to_string());
                    creep.memory().set("harvesting", true);
                }
                None => (),
            }
        }
    }

    if creep.memory().bool("harvesting") && creep.memory().string("source").unwrap().is_some() {
        creep_actions::harvest(&creep)
    } else {
        if creep.energy() == 0 {
            creep.memory().set("harvesting", true);
        } else {
            spend_energy(creep, available_jobs)
        }
    }
}

fn spend_energy(creep: screeps::objects::Creep, available_jobs: &mut job_manager::AvailableJobs) {
    if creep_actions::repair_local_road(&creep) {
        return;
    }

    let room = creep.room().unwrap();
    let current_room_name = room.name();

    let default = vec![];
    let construction_sites = available_jobs
        .jobs_by_room
        .get(&current_room_name)
        .map_or(&default, |room_jobs| &room_jobs.construction_jobs);

    let structures = room.find(find::STRUCTURES);
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

    if towers.len() > 0 {
        creep_actions::fill(&creep, &towers[0]);
    } else if extensions.len() > 0 {
        creep_actions::fill(&creep, &extensions[0]);
    } else if room.storage().map_or(1000000, |store| store.energy()) < 2000 {
        let the_storage = room.storage().unwrap();
        let as_structure = screeps::objects::Structure::Storage(the_storage);
        creep_actions::fill(&creep, &as_structure);
    } else if construction_sites.len() > 0 {
        creep_actions::build(creep, &construction_sites[0]);
    } else {
        creep_actions::upgrade_controller(creep, &room.controller().unwrap());
    };
}
