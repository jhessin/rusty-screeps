use log::*;
use screeps::{
  find,
  objects::{Creep, HasPosition, HasStore, RoomObjectProperties},
  ResourceType, ReturnCode,
};

use crate::fsm::CreepState;
use crate::role_harvester;

/// fn build(&Creep) -> bool
/// Tell the given creep to build if there is anything to build.
/// Return false if there is nothing to build or if there is any other error.
pub fn build(creep: &Creep) -> bool {
  // find buildings
  let targets = creep.room().find(find::CONSTRUCTION_SITES);
  if targets.len() > 0 {
    let target = &targets[0];
    if creep.pos().is_near_to(target) {
      let r = creep.build(&target);
      if r != ReturnCode::Ok {
        creep.set_status("Error building");
        warn!("Couldn't build: {:?}", r);
        false
      } else {
        creep.set_status("Building");
        true
      }
    } else {
      creep.set_status("Moving to site");
      creep.move_to(target);
      true
    }
  } else {
    creep.set_status("Nothing to build");
    false
  }
}

/// role_builder::run(&Creep) -> bool
/// This is the main role function for the builder creep.
/// Only administrative code should go in here.
/// i.e. setting flags in memory, etc.
/// Other functions should do the actual work.
/// Should return true if the creep is given a task to do.
/// Should return false if the creep can be considered as idle.
pub fn run(creep: &Creep) -> bool {
  creep.set_status("Builder Pending");
  let building = creep.memory().bool("building");

  if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
    // full of energy and need to build
    creep.memory().set("building", true);
    // creep.say("🚧 build", false);
    creep.set_status("Building");
    build(&creep)
  } else if building && creep.store_of(ResourceType::Energy) == 0 {
    // building but out of energy
    creep.memory().set("building", false);
    // creep.say("🌸 harvest", false);
    creep.set_status("Harvesting");
    role_harvester::run(&creep)
  } else {
    build(&creep)
  }
}
