use log::*;

use std::collections::HashMap;
use screeps::{find, prelude::*, ResourceType, ReturnCode};


pub fn run_harvester(creep: screeps::objects::Creep) {

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
        }
    } else {
        if creep.memory().get::<screeps::objects::Source>("source").unwrap().is_none() {
            creep.memory().set("harvesting", true);

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
                let source_opt = creep.memory().get::<screeps::objects::Source>("source").unwrap();
                match source_opt {
                    Some(source) => {
                        let count = worked_resources.entry(source.id()).or_insert(0);
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
            creep.memory().set("source", lowest_used_source.unwrap());
        }
    }

    // second check: if we're harvesting, go do that. if we're using energy, go do that

    if creep.memory().bool("harvesting") {

        let source = creep.memory().get::<screeps::objects::Source>("source").unwrap().unwrap();

        if creep.pos().is_near_to(&source) {
            let r = creep.harvest(&source);
            if r != ReturnCode::Ok {
                warn!("couldn't harvest: {:?}", r)
            }
        } else {
            creep.move_to(&source);
        }

    } else {

        let mem = screeps::memory::root();
        let home_room = mem.get::<screeps::objects::Room>("home_room").unwrap().unwrap();
        let master_controller = home_room.controller().unwrap();

        let r = creep.upgrade_controller(&master_controller);
        if r == ReturnCode::NotInRange {
            creep.move_to(&master_controller);
        } else if r != ReturnCode::Ok {
            warn!("couldn't upgrade: {:?}", r);
        }

    }

}