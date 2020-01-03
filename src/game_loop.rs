use log::*;
use screeps::memory::MemoryReference;
use screeps::{prelude::*, Part, ReturnCode, SpawnOptions};

use crate::util::cleanup_memory;

use crate::harvester;

pub fn game_loop() {
    debug!("loop starting! CPU: {}", screeps::game::cpu::get_used());

    let mem = screeps::memory::root();
    mem.set("worked_rooms", vec!["W44S28"]);
    mem.set("home_room", "W44S28");

    debug!("running spawns");
    for spawn in screeps::game::spawns::values() {
        debug!("running spawn {}", spawn.name());
        let body = [Part::Move, Part::Move, Part::Carry, Part::Work];

        if spawn.energy() >= body.iter().map(|p| p.cost()).sum() {
            // create a unique name, spawn.
            let name_base = screeps::game::time();
            let mut additional = 0;
            let res = loop {
                let name = format!("{}-{}", name_base, additional);
                let mem = MemoryReference::new();
                mem.set("type", "harvester");
                let options = SpawnOptions::new().memory(mem);
                let res = spawn.spawn_creep_with_options(&body, &name, &options);

                if res == ReturnCode::NameExists {
                    additional += 1;
                } else {
                    break res;
                }
            };

            if res != ReturnCode::Ok {
                warn!("couldn't spawn: {:?}", res);
            }
        }
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
        }
    }

    let time = screeps::game::time();

    if time % 32 == 3 {
        info!("running memory cleanup");
        cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
    }

    info!("done! cpu: {}", screeps::game::cpu::get_used())
}
