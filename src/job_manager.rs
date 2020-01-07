/*
a new approach to work - perform one search to identify all the jobs that need
doing, and then allocate available creeps to work

when searching for jobs, we can build an in-memory struct that stores a number of
lists of jobs. we should also scope this by room, allowing creeps to more easily
select a job from their room. finally each job should also have a position, so
when a creep selects one it can someday pick a close one not a far one.
*/

//use log::*;
use screeps::{find, prelude::*, ObjectId};
use std::collections::HashMap;
use std::rc::Rc;

pub struct RoomJobs {
    pub harvest_jobs: std::vec::Vec<Rc<screeps::objects::Source>>,
}

pub struct AvailableJobs {
    pub reserve_jobs: std::vec::Vec<String>,
    pub jobs_by_room: std::collections::HashMap<screeps::local::RoomName, RoomJobs>,
}

impl AvailableJobs {
    pub fn new() -> AvailableJobs {
        AvailableJobs {
            reserve_jobs: vec![],
            jobs_by_room: std::collections::HashMap::new(),
        }
    }

    pub fn pop_reserve_job(&mut self) -> Option<String> {
        self.reserve_jobs.pop()
    }

    pub fn pop_harvest_job_for_room(
        &mut self,
        room_name: screeps::local::RoomName,
    ) -> Option<Rc<screeps::objects::Source>> {
        match self.jobs_by_room.get_mut(&room_name) {
            Some(room_jobs) => room_jobs.harvest_jobs.pop(),
            None => None,
        }
    }

    pub fn pop_harvest_job_any(&mut self) -> Option<Rc<screeps::objects::Source>> {
        let mut maybe_job = None;
        let mut room_names = vec![];
        for rn in self.jobs_by_room.keys().copied() {
            room_names.push(rn);
        }
        for room_name in room_names {
            let room_jobs_opt = self.jobs_by_room.get_mut(&room_name);
            match room_jobs_opt {
                Some(room_jobs) => {
                    if room_jobs.harvest_jobs.len() > 0 {
                        maybe_job = room_jobs.harvest_jobs.pop();
                        break;
                    }
                }
                None => (),
            }
        }
        return maybe_job;
    }
}

pub fn build_job_set() -> AvailableJobs {
    let mut available_jobs = AvailableJobs::new();

    find_reserve_jobs(&mut available_jobs);
    find_harvest_jobs(&mut available_jobs);

    return available_jobs;
}

fn find_reserve_jobs(availabe_jobs: &mut AvailableJobs) {
    let mem = screeps::memory::root();
    let reserved_rooms: std::vec::Vec<String> = mem.arr("reserved_rooms").unwrap().unwrap();

    let mut worked_reservations = HashMap::new();

    for room in reserved_rooms {
        worked_reservations.entry(room).or_insert(0);
    }

    for creep in screeps::game::creeps::values() {
        let target_opt = creep.memory().string("reserve_target").unwrap();
        match target_opt {
            Some(target) => {
                let count = worked_reservations.entry(target).or_insert(0);
                *count += 1;
            }
            None => (),
        };
    }

    for (room_str, count) in worked_reservations {
        if count == 0 {
            availabe_jobs.reserve_jobs.push(room_str);
        }
    }
}

fn find_harvest_jobs(availabe_jobs: &mut AvailableJobs) {
    let mem = screeps::memory::root();
    let worked_rooms: Vec<String> = mem.arr("worked_rooms").unwrap().unwrap();

    let mut worked_resources = HashMap::new();

    for room in worked_rooms {
        let room_name = screeps::local::RoomName::new(&room).unwrap();
        let room_res = screeps::game::rooms::get(room_name);
        match room_res {
            Some(room) => {
                for source in room.find(find::SOURCES) {
                    worked_resources.entry(source.id()).or_insert(0);
                }
            }
            _ => (),
        };
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

    let harvester_target = mem.i32("harvesters").unwrap().unwrap();
    let worked_resource_count = worked_resources.values().len();

    for (source_id, count) in worked_resources {
        if count < harvester_target / worked_resource_count as i32 {
            let source = source_id.try_resolve().unwrap().unwrap();
            let in_room = availabe_jobs
                .jobs_by_room
                .entry(source.room().unwrap().name())
                .or_insert(RoomJobs {
                    harvest_jobs: vec![],
                });
            in_room.harvest_jobs.push(Rc::new(source));
        }
    }
}
