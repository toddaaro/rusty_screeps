use log::*;
use screeps::{prelude::*, ResourceType, ReturnCode};
use std::cmp::min;

pub fn upgrade_controller(
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

pub fn build(creep: screeps::objects::Creep, target_site: &screeps::objects::ConstructionSite) {
    let r = creep.build(target_site);
    if r == ReturnCode::NotInRange {
        creep.move_to(target_site);
    } else if r != ReturnCode::Ok {
        warn!("couldn't build: {:?}", r);
    }
}

pub fn fill(creep: screeps::objects::Creep, fill_target: &screeps::objects::Structure) {
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

pub fn reserve(creep: screeps::objects::Creep, target: &screeps::objects::StructureController) {
    let r = creep.reserve_controller(target);
    if r == ReturnCode::NotInRange {
        creep.move_to(target);
    } else if r != ReturnCode::Ok {
        warn!("couldn't reserve: {:?}", r);
    }
}

pub fn repair_local_road(creep: &screeps::objects::Creep) -> bool {
    let position = creep.pos();
    let structures = position.look_for(screeps::look::STRUCTURES);

    for structure in structures {
        match structure.as_attackable() {
            Some(attackable) => {
                if attackable.hits() + 100 < attackable.hits_max() {
                    creep.repair(&structure);
                    return true;
                }
            }
            None => (),
        }
    }
    return false;
}
