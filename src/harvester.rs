use log::*;
use screeps::{find, prelude::*, ObjectId, ResourceType, ReturnCode};
use std::collections::HashMap;

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
        } else if creep.memory().string("source").unwrap().is_none() {
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
            }

            for creep in screeps::game::creeps::values() {
                let source_opt = creep.memory().string("source").unwrap();
                match source_opt {
                    Some(source) => {
                        let source_id: ObjectId<screeps::objects::Source> = source.parse().unwrap();
                        let count = worked_resources.entry(source_id).or_insert(0);
                        *count += 1;
                    }
                    None => (),
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
            creep.memory().set("source", target_source.id().to_string());
            creep.memory().set("harvesting", true);
        }
    }

    // second check: if we're harvesting, go do that. if we're using energy, go do that

    if creep.memory().bool("harvesting") {
        let source_id_raw = creep.memory().string("source");
        let source_id: ObjectId<screeps::objects::Source> =
            source_id_raw.unwrap().unwrap().parse().unwrap();
        let source = screeps::game::get_object_typed(source_id).unwrap().unwrap();

        let near_to_result = creep.pos().is_near_to(&source);
        if near_to_result {
            let r = creep.harvest(&source);
            if r != ReturnCode::Ok {
                warn!("couldn't harvest: {:?}", r)
            }
        } else {
            creep.move_to(&source);
        }
    } else {
        if creep.energy() == 0 {
            creep.memory().set("harvesting", true);
        } else {
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
