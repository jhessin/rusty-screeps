use log::*;
use screeps::{
  game::{self, cpu},
  objects::{Creep, Structure},
};

use stdweb::js;

mod generator_builder;
mod generator_harvester;
mod generator_upgrader;

pub mod fsm;
pub mod role_builder;
pub mod role_harvester;
pub mod role_tower;
pub mod role_upgrader;
pub mod tools;

const NUM_UPGRADERS: usize = 1;
const NUM_HARVESTERS: usize = 2;
const NUM_BUILDERS: usize = 3;

mod logging;
mod memory;

fn main() {
  logging::setup_logging(logging::Info);

  js! {
      var game_loop = @{game_loop};

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

#[allow(unused_must_use)]
fn game_loop() {
  debug!("loop starting! CPU: {}", cpu::get_used());

  debug!("running spawns");
  for spawn in game::spawns::values() {
    debug!("running spawn {}", spawn.name());

    let mut harvesters: Vec<Creep> = vec![];
    let mut builders: Vec<Creep> = vec![];
    let mut upgraders: Vec<Creep> = vec![];

    for creep in game::creeps::values() {
      if creep.name().starts_with("Harvester") {
        harvesters.push(creep);
      } else if creep.name().starts_with("Builder") {
        builders.push(creep);
      } else if creep.name().starts_with("Upgrader") {
        upgraders.push(creep);
      } else {
        harvesters.push(creep);
      }
    }
    info!("harvesters: {}", harvesters.len());
    info!("builders: {}", builders.len());
    info!("upgraders: {}", upgraders.len());

    if harvesters.len() < NUM_HARVESTERS {
      generator_harvester::run(&spawn);
    } else if builders.len() < NUM_BUILDERS {
      generator_builder::run(&spawn);
    } else if upgraders.len() < NUM_UPGRADERS {
      generator_upgrader::run(&spawn);
    }
  }

  debug!("running structures");
  for s in game::structures::values().iter() {
    if let Structure::Tower(tower) = s {
      role_tower::run(tower);
    }
  }

  debug!("running creeps");
  for creep in screeps::game::creeps::values() {
    let name = creep.name();
    debug!("running creep {}", name);
    if creep.spawning() {
      continue;
    }

    if name.starts_with("Harvester") {
      role_harvester::run(&creep)
        || role_builder::run(&creep)
        || role_upgrader::run(&creep);
    } else if name.starts_with("Upgrader") {
      role_upgrader::run(&creep);
    } else if name.starts_with("Builder") {
      role_builder::run(&creep)
        || role_harvester::run(&creep)
        || role_upgrader::run(&creep);
    } else {
      role_harvester::run(&creep)
        || role_builder::run(&creep)
        || role_upgrader::run(&creep);
    }

    let time = screeps::game::time();

    if time % 32 == 3 {
      info!("running memory cleanup");
      memory::cleanup_memory()
        .expect("expected Memory.creeps format to be a regular memory object");
    }

    // info!("done! cpu: {}", screeps::game::cpu::get_used())
  }
}
