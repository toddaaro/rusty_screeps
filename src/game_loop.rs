use crate::util::cleanup_memory;
use crate::{filler, goals, harvester, job_manager, reserver, settler, spawner, tower, upgrader};
use log::*;
use screeps::{find, prelude::*};

pub fn game_loop() {
    debug!("loop starting! CPU: {}", screeps::game::cpu::get_used());

    goals::set_goals();
    let mut available_jobs = job_manager::build_job_set();

    debug!("running towers");
    let mut towers: std::vec::Vec<screeps::objects::StructureTower> = vec![];
    for room in screeps::game::rooms::values() {
        let structures = room.find(find::STRUCTURES);
        for my_structure in structures {
            match my_structure {
                screeps::Structure::Tower(my_tower) => {
                    towers.push(my_tower);
                }
                _ => (),
            };
        }
    }
    for my_tower in towers {
        tower::run_tower(my_tower);
    }

    debug!("running spawns");
    for spawn in screeps::game::spawns::values() {
        spawner::run_spawn(spawn);
    }

    debug!("running creeps");
    for creep in screeps::game::creeps::values() {
        let name = creep.name();
        debug!("running creep {}", name);
        if creep.spawning() {
            continue;
        }

        let creep_type = creep.memory().string("type").unwrap();
        if creep_type == Some("settler".to_string()) {
            settler::run_settler(creep, &mut available_jobs);
        } else if creep_type == Some("harvester".to_string()) {
            harvester::run_harvester(creep, &mut available_jobs);
        } else if creep_type == Some("filler".to_string()) {
            filler::run_filler(creep);
        } else if creep_type == Some("reserver".to_string()) {
            reserver::run_reserver(creep, &mut available_jobs);
        } else if creep_type == Some("upgrader".to_string()) {
            upgrader::run_upgrader(creep);
        }
    }

    let time = screeps::game::time();

    if time % 32 == 3 {
        info!("running memory cleanup");
        cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
    }

    info!("done! cpu: {}", screeps::game::cpu::get_used())
}
