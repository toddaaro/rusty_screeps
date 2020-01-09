pub fn set_goals() {
    let mem = screeps::memory::root();
    mem.set("home_room", "W44S28");
    mem.set("reserved_rooms", vec!["W43S28", "W44S29"]);
    mem.set("worked_rooms", vec!["W44S28", "W43S28", "W44S29"]);

    mem.path_set("spawn_goals.W44S28.harvester", 13);
    mem.path_set("spawn_goals.W44S28.filler", 2);
    mem.path_set("spawn_goals.W44S28.reserver", 2);
    mem.path_set("spawn_goals.W44S28.upgrader", 2);
    mem.path_set("spawn_goals.W44S28.settler", 2);
}
