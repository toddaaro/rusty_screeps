use log::*;
use screeps::{find, prelude::*, ObjectId, ResourceType, ReturnCode};

pub fn run_filler(creep: screeps::objects::Creep) {
    let name = creep.name();
    debug!("running creep {}", name);

    if creep.spawning() {
        return;
    }

    let mem = screeps::memory::root();
    let home_room_name_str = mem.string("home_room").unwrap().unwrap();
    let home_room_name = screeps::local::RoomName::new(&home_room_name_str).unwrap();
    let home_room = screeps::game::rooms::get(home_room_name).unwrap();
    let storage = home_room.storage().unwrap();

    if creep.memory().bool("withdrawing") {
        if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
            creep.memory().set("withdrawing", false);
        } else {
            let r = creep.withdraw_all(&storage, ResourceType::Energy);
            if r == ReturnCode::NotInRange {
                creep.move_to(&storage);
            } else if r != ReturnCode::Ok {
                warn!("couldn't withdraw: {:?}", r);
            }
        }
    } else {
        if creep.energy() == 0 {
            creep.memory().del("fill_target");
            creep.memory().set("withdrawing", true);
        } else if creep.memory().string("fill_target").unwrap().is_none() {
            let structures = home_room.find(find::STRUCTURES);
            let mut fillable: std::vec::Vec<screeps::Structure> = vec![];
            for my_structure in structures {
                match my_structure.as_has_store() {
                    Some(with_store) => {
                        if with_store.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                            fillable.push(my_structure);
                        }
                    }
                    None => (),
                }
            }

            let fill_target = fillable[0].id();
            creep.memory().set("fill_target", fill_target.to_string());
        } else {
            let fill_target_str = creep.memory().string("fill_target");
            let fill_target_id: ObjectId<screeps::objects::Structure> =
                fill_target_str.unwrap().unwrap().parse().unwrap();
            let fill_target_structure = screeps::game::get_object_typed(fill_target_id)
                .unwrap()
                .unwrap();
            let fill_target = fill_target_structure.as_transferable().unwrap();

            let r = creep.transfer_all(fill_target, ResourceType::Energy);
            if r == ReturnCode::NotInRange {
                creep.move_to(fill_target);
            } else if r == ReturnCode::Full {
                creep.memory().del("fill_target");
            } else if r != ReturnCode::Ok {
                warn!("couldn't transfer: {:?}", r);
            }
        }
    }
}
