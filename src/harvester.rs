use log::*;
use screeps::{find, prelude::*, ObjectId, ResourceType, ReturnCode};
use std::cmp::min;
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
            spend_energy(creep)
        }
    }
}

fn spend_energy(creep: screeps::objects::Creep) {
    let mem = screeps::memory::root();
    let home_room_name_str = mem.string("home_room").unwrap().unwrap();
    let home_room_name = screeps::local::RoomName::new(&home_room_name_str).unwrap();
    let home_room = screeps::game::rooms::get(home_room_name).unwrap();

    let construction_sites = home_room.find(find::MY_CONSTRUCTION_SITES);

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

    if towers.len() > 0 {
        fill(creep, &towers[0]);
    } else if extensions.len() > 0 {
        fill(creep, &extensions[0]);
    } else if construction_sites.len() > 0 {
        build(creep, &construction_sites[0]);
    } else {
        upgrade_controller(creep, &home_room.controller().unwrap());
    };
}

fn upgrade_controller(
    creep: screeps::objects::Creep,
    controller: &screeps::objects::StructureController,
) {
    let r = creep.upgrade_controller(controller);
    if r == ReturnCode::NotInRange {
        creep.move_to(controller);
    } else if r != ReturnCode::Ok {
        warn!("couldn't upgrade: {:?}", r);
    }
}

fn build(creep: screeps::objects::Creep, target_site: &screeps::objects::ConstructionSite) {
    let r = creep.build(target_site);
    if r == ReturnCode::NotInRange {
        creep.move_to(target_site);
    } else if r != ReturnCode::Ok {
        warn!("couldn't build: {:?}", r);
    }
}

fn fill(creep: screeps::objects::Creep, fill_target: &screeps::objects::Structure) {
    let transferable = fill_target.as_transferable().unwrap();
    let has_store = fill_target.as_has_store().unwrap();

    let empty_space = has_store.store_free_capacity(Some(ResourceType::Energy));
    let creep_energy = creep.energy();
    let amount = min(creep_energy, empty_space);

    let r = creep.transfer_amount(transferable, ResourceType::Energy, amount);
    if r == ReturnCode::NotInRange {
        creep.move_to(fill_target);
    } else if r == ReturnCode::Full {
        creep.memory().del("fill_target");
    } else if r != ReturnCode::Ok {
        warn!("couldn't transfer: {:?}", r);
    }
}
