use log::*;
use screeps::{find, prelude::*};

pub fn run_tower(tower: screeps::objects::StructureTower) {
    debug!("running tower {:?}", tower.id());

    let room = tower.room().unwrap();
    let targets = room.find(find::HOSTILE_CREEPS);
    if targets.len() > 0 {
        tower.attack(&targets[0]);
    }
}
