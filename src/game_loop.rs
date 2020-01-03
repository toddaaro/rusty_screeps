use log::*;
use screeps::{find, prelude::*, Part, ResourceType, ReturnCode, RoomObjectProperties, SpawnOptions}; // game
use screeps::memory::MemoryReference;

use crate::util::{cleanup_memory};

use crate::harvester;


pub fn game_loop() {
    debug!("loop starting! CPU: {}", screeps::game::cpu::get_used());

    debug!("running the world setup, if not done");
    let mem = screeps::memory::root();
    //if mem.get::<Vec<String>>("worked_rooms").unwrap().is_none() {
    warn!("setting the global goals!");
    let worked_rooms = vec!["W44S28"];
    mem.set("worked_rooms", worked_rooms);
    mem.set("home_room", "W44S28");
    //}

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

        warn!("unwrapping creep type");
        let creep_type = creep.memory().string("type").unwrap();
        warn!("found type: {:?} for creep {:?}", creep_type, creep.name());
        if creep_type == Some("harvester".to_string()) {
            harvester::run_harvester(creep);
        }
        else {
            if creep.memory().bool("harvesting") {
                if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
                    creep.memory().set("harvesting", false);
                }
            } else {
                if creep.store_used_capacity(None) == 0 {
                    creep.memory().set("harvesting", true);                
                }
            }

            if creep.memory().bool("harvesting") {

                let source = &creep.room().find(find::SOURCES)[0];

                if creep.pos().is_near_to(source) {
                    let r = creep.harvest(source);
                    if r != ReturnCode::Ok {
                        warn!("couldn't harvest: {:?}", r);
                    }
                } else {
                    creep.move_to(source);
                }
            } else {
                if let Some(c) = creep.room().controller() {
                    let r = creep.upgrade_controller(&c);
                    if r == ReturnCode::NotInRange {
                        creep.move_to(&c);
                    } else if r != ReturnCode::Ok {
                        warn!("couldn't upgrade: {:?}", r);
                    }
                } else {
                    warn!("creep room has no controller!");
                }
            }
        }
    }

    let time = screeps::game::time();

    if time % 32 == 3 {
        info!("running memory cleanup");
        cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
    }

    info!("done! cpu: {}", screeps::game::cpu::get_used())
}
