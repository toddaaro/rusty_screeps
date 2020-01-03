use log::*;

use std::collections::HashMap;
use screeps::{find, prelude::*, ResourceType, ReturnCode, ObjectId};


pub fn run_harvester(creep: screeps::objects::Creep) {

    let name = creep.name();
    debug!("running creep {}", name);

    if creep.spawning() {
        return;
    }

    // first check: did we fill up with energy or did we run out? if so, update what we're doing

    if creep.memory().bool("harvesting") {
        warn!("checking if harvesting");
        if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
            creep.memory().set("harvesting", false);
            creep.memory().del("source");
        } else if creep.memory().string("source").unwrap().is_none() {            
            warn!("none found, trying to set the target");
            // stored in the memory is a manually set list of all the rooms we work
            let mem = screeps::memory::root();
            let worked_rooms: Vec<String> = mem.arr("worked_rooms").unwrap().unwrap();

            // to figure out where we should go, we look over all our creeps and build a map
            // of what resources are being worked. then we pick a resource that has the lowest
            // count. initializing requires finding all sources in our active rooms.
            let mut worked_resources = HashMap::new();                                                

            for room in worked_rooms {
                let room_name = screeps::local::RoomName::new(&room).unwrap();
                let room = screeps::game::rooms::get(room_name).unwrap();

                for source in room.find(find::SOURCES) {
                    worked_resources.entry(source.id()).or_insert(0);
                }
            };
            
            for creep in screeps::game::creeps::values() {
                let source_opt = creep.memory().string("source").unwrap();
                match source_opt {
                    Some(source) => {
                        let source_id: ObjectId<screeps::objects::Source> = source.parse().unwrap();
                        let count = worked_resources.entry(source_id).or_insert(0);
                        *count += 1;
                    }
                    None => ()
                };
            }

            let mut lowest_used_source: Option<screeps::objects::Source> = None;
            let mut lowest_used_count = 9001;
            for (source_id, count) in worked_resources {
                let source = source_id.try_resolve().unwrap().unwrap();
                if count < lowest_used_count {
                    lowest_used_source = Some(source);
                    lowest_used_count = count;
                }
            }
            let target_source = lowest_used_source.unwrap();
            warn!("setting target source for {:?} to {:?}", creep.name(), target_source.id());
            creep.memory().set("source", target_source.id().to_string());
            warn!("memory now showing {:?} for target source", creep.memory().string("source").unwrap());
            creep.memory().set("harvesting", true);
        }
    }

    // second check: if we're harvesting, go do that. if we're using energy, go do that

    if creep.memory().bool("harvesting") {
        warn!("trying to harvest");

        
        let source_id_raw = creep.memory().string("source");

        let mut source_id_opt = None;
        match source_id_raw {
            Ok(v) => {
                warn!("raw source id ok");
                source_id_opt = v;
            },
            Err(v) => {
                warn!("raw source id not ok: {:?}", v);
                source_id_opt = None;
            }
        };

        warn!("got raw source id");
        let source_id: ObjectId<screeps::objects::Source> = source_id_opt.unwrap().parse().unwrap();
        warn!("got source id");
        let source = screeps::game::get_object_typed(source_id).unwrap().unwrap();
        warn!("got source");

        // let raw_source = creep.memory().get::<screeps::objects::Source>("source");
        // let mut source_opt = None;
        // match raw_source {
        //     Ok(v) => {
        //         warn!("raw source ok");
        //         source_opt = v;
        //     },
        //     Err(v) => {
        //         warn!("raw source not ok: {:?}", v);
        //         source_opt = None;
        //     }
        // };
        // let source = source_opt.unwrap();

        let near_to_result = creep.pos().is_near_to(&source);
        warn!("got near to");

        if near_to_result {
            warn!("am near the source");
            let r = creep.harvest(&source);
            if r != ReturnCode::Ok {
                warn!("couldn't harvest: {:?}", r)
            }
        } else {
            warn!("not near the source");
            creep.move_to(&source);
        }

    } else {
        warn!("continue harvesting?");        
        if creep.energy() == 0 {
            warn!("found creep.energy() {:?} so setting haresting to true", creep.energy());
            creep.memory().set("harvesting", true);
        } else {
            warn!("creep has energy so trying to spend it");
            let mem = screeps::memory::root();
            let home_room_name_str = mem.string("home_room").unwrap().unwrap();
            let home_room_name = screeps::local::RoomName::new(&home_room_name_str).unwrap();
            let home_room = screeps::game::rooms::get(home_room_name).unwrap();
            let master_controller = home_room.controller().unwrap();

            let r = creep.upgrade_controller(&master_controller);
            if r == ReturnCode::NotInRange {
                creep.move_to(&master_controller);
            } else if r != ReturnCode::Ok {
                warn!("couldn't upgrade: {:?}", r);
            }
        }
    }
}
