use log::*;
use screeps::objects::Attackable;
use screeps::{find, prelude::*};

pub fn run_tower(tower: screeps::objects::StructureTower) {
    debug!("running tower {:?}", tower.id());

    let room = tower.room().unwrap();
    let targets = room.find(find::HOSTILE_CREEPS);
    if targets.len() > 0 {
        tower.attack(&targets[0]);
    }

    let my_structures = room.find(find::MY_STRUCTURES);
    let mut repair_targets: std::vec::Vec<screeps::objects::OwnedStructure> = vec![];
    for structure in my_structures {
        if structure.hits() + 800 < structure.hits_max() {
            repair_targets.push(structure);
        }
    }
    if repair_targets.len() > 0 {
        tower.repair(&repair_targets[0]);
    }
}
