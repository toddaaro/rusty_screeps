use std::collections::HashSet;

use log::*;


pub fn cleanup_memory() -> Result<(), Box<dyn std::error::Error>> {
    let alive_creeps: HashSet<String> = screeps::game::creeps::keys().into_iter().collect();

    let screeps_memory = match screeps::memory::root().dict("creeps")? {
        Some(v) => v,
        None => {
            warn!("not cleaning game creep memory: no Memory.creeps dict");
            return Ok(());
        }
    };

    for mem_name in screeps_memory.keys() {
        if !alive_creeps.contains(&mem_name) {
            debug!("cleaning up creep memory of dead creep {}", mem_name);
            screeps_memory.del(&mem_name);
        }
    }

    Ok(())
}

// currently dead code for initializing the prng without using any system entropy? dunno what the deal is with none of that 
// working at all
// pub fn transform_u32_to_array_of_u8_x4(x:u32) -> [u8;16] {
//     let b1 : u8 = ((x >> 24) & 0xff) as u8;
//     let b2 : u8 = ((x >> 16) & 0xff) as u8;
//     let b3 : u8 = ((x >> 8) & 0xff) as u8;
//     let b4 : u8 = (x & 0xff) as u8;
//     return [b1, b2, b3, b4, b1, b2, b3, b4, b1, b2, b3, b4, b1, b2, b3, b4]
// }
