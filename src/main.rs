use stdweb::js;

mod creep_actions;
mod filler;
mod game_loop;
mod goals;
mod harvester;
mod job_manager;
mod logging;
mod reserver;
mod settler;
mod spawner;
mod tower;
mod upgrader;
mod util;

fn main() {
    logging::setup_logging(logging::Info);

    js! {
        var game_loop = @{game_loop::game_loop};

        module.exports.loop = function() {
            // Provide actual error traces.
            try {
                game_loop();
            } catch (error) {
                // console_error function provided by 'screeps-game-api'
                console_error("caught exception:", error);
                if (error.stack) {
                    console_error("stack trace:", error.stack);
                }
                console_error("resetting VM next tick.");
                // reset the VM since we don't know if everything was cleaned up and don't
                // want an inconsistent state.
                module.exports.loop = wasm_initialize;
            }
        }
    }
}
