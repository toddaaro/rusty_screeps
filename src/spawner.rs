use log::*;
use screeps::memory::MemoryReference;
use screeps::{prelude::*, Part, ReturnCode, SpawnOptions};
use std::convert::TryFrom;

pub fn run_spawn(spawn: screeps::objects::StructureSpawn) {
    debug!("running spawn {}", spawn.name());

    // find out what our current goals are
    let mem = screeps::memory::root();
    let harvester_goal = mem.i32("harvesters").unwrap().unwrap();
    let filler_goal = mem.i32("fillers").unwrap().unwrap();
    let reserver_goal = mem.i32("reservers").unwrap().unwrap();
    let upgrader_goal = mem.i32("upgraders").unwrap().unwrap();
    let settler_goal = mem.i32("settlers").unwrap().unwrap();

    let current_creeps = screeps::game::creeps::values();
    let mut current_harvesters: std::vec::Vec<screeps::objects::Creep> = vec![];
    let mut current_fillers: std::vec::Vec<screeps::objects::Creep> = vec![];
    let mut current_reservers: std::vec::Vec<screeps::objects::Creep> = vec![];
    let mut current_upgraders: std::vec::Vec<screeps::objects::Creep> = vec![];
    let mut current_settlers: std::vec::Vec<screeps::objects::Creep> = vec![];

    for creep in current_creeps {
        let creep_type = creep.memory().string("type").unwrap().unwrap();
        match creep_type.as_ref() {
            "harvester" => current_harvesters.push(creep),
            "filler" => current_fillers.push(creep),
            "reserver" => current_reservers.push(creep),
            "upgrader" => current_upgraders.push(creep),
            "settler" => current_settlers.push(creep),
            _ => (),
        }
    }

    if current_settlers.len() < usize::try_from(settler_goal).unwrap() {
        build_settler(spawn);
    } else if current_fillers.len() < usize::try_from(filler_goal).unwrap() {
        build_filler(spawn);
    } else if current_harvesters.len() < usize::try_from(harvester_goal).unwrap() {
        build_harvester(spawn);
    } else if current_reservers.len() < usize::try_from(reserver_goal).unwrap() {
        build_reserver(spawn);
    } else if current_upgraders.len() < usize::try_from(upgrader_goal).unwrap() {
        build_upgrader(spawn);
    }
}

fn build_settler(spawn: screeps::objects::StructureSpawn) {
    let small = vec![Part::Move, Part::Move, Part::Carry, Part::Work];
    if dry_run_build(&spawn, &small) {
        build_creep(spawn, "harvester", small);
    }
}

fn build_harvester(spawn: screeps::objects::StructureSpawn) {
    let small = vec![Part::Move, Part::Move, Part::Carry, Part::Work];
    let medium = vec![
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Carry,
        Part::Carry,
        Part::Work,
        Part::Work,
        Part::Work,
    ];
    let big = vec![
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Work,
        Part::Work,
        Part::Work,
        Part::Work,
    ];

    if dry_run_build(&spawn, &big) {
        build_creep(spawn, "harvester", big);
    } else if dry_run_build(&spawn, &medium) {
        build_creep(spawn, "harvester", medium);
    } else if dry_run_build(&spawn, &small) {
        build_creep(spawn, "harvester", small);
    }
}

fn build_filler(spawn: screeps::objects::StructureSpawn) {
    let small = vec![Part::Move, Part::Carry, Part::Carry];
    let medium = vec![
        Part::Move,
        Part::Move,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Carry,
    ];
    let large = vec![
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Carry,
    ];

    if dry_run_build(&spawn, &large) {
        build_creep(spawn, "filler", large);
    } else if dry_run_build(&spawn, &medium) {
        build_creep(spawn, "filler", medium);
    } else if dry_run_build(&spawn, &small) {
        build_creep(spawn, "filler", small);
    }
}

fn build_reserver(spawn: screeps::objects::StructureSpawn) {
    let small = vec![Part::Move, Part::Move, Part::Claim, Part::Claim];
    if dry_run_build(&spawn, &small) {
        build_creep(spawn, "reserver", small);
    }
}

fn build_upgrader(spawn: screeps::objects::StructureSpawn) {
    let small = vec![Part::Move, Part::Carry, Part::Work];
    let medium = vec![
        Part::Move,
        Part::Move,
        Part::Carry,
        Part::Carry,
        Part::Work,
        Part::Work,
    ];
    let large = vec![
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Work,
        Part::Work,
        Part::Work,
        Part::Work,
        Part::Work,
        Part::Work,
    ];

    if dry_run_build(&spawn, &large) {
        build_creep(spawn, "upgrader", large);
    } else if dry_run_build(&spawn, &medium) {
        build_creep(spawn, "upgrader", medium);
    } else if dry_run_build(&spawn, &small) {
        build_creep(spawn, "upgrader", small);
    }
}

fn build_creep(
    spawn: screeps::objects::StructureSpawn,
    creep_type: &str,
    body: std::vec::Vec<screeps::constants::Part>,
) {
    // create a unique name, spawn.
    let name_base = screeps::game::time();
    let mut additional = 0;
    let res = loop {
        let name = format!("{}-{}", name_base, additional);
        let mem = MemoryReference::new();
        mem.set("type", creep_type);
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

fn dry_run_build(
    spawn: &screeps::objects::StructureSpawn,
    body: &std::vec::Vec<screeps::constants::Part>,
) -> bool {
    let name_base = screeps::game::time();
    let mut additional = 0;
    let res = loop {
        let name = format!("{}-{}", name_base, additional);
        let options = SpawnOptions::new().dry_run(true);
        let res = spawn.spawn_creep_with_options(&body, &name, &options);

        if res == ReturnCode::NameExists {
            additional += 1;
        } else {
            break res;
        }
    };
    return res == ReturnCode::Ok;
}
