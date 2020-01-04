use crate::util::cleanup_memory;
use crate::{filler, harvester, spawner, tower};
use log::*;
use screeps::{find, prelude::*};

pub fn game_loop() {
    debug!("loop starting! CPU: {}", screeps::game::cpu::get_used());

    let mem = screeps::memory::root();
    mem.set("worked_rooms", vec!["W44S28"]);
    mem.set("home_room", "W44S28");
    mem.set("harvesters", 8);
    mem.set("fillers", 0);

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
        if creep_type == Some("harvester".to_string()) {
            harvester::run_harvester(creep);
        } else if creep_type == Some("filler".to_string()) {
            filler::run_filler(creep);
        }
    }

    let time = screeps::game::time();

    if time % 32 == 3 {
        info!("running memory cleanup");
        cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
    }

    info!("done! cpu: {}", screeps::game::cpu::get_used())
}
