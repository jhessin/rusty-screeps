use log::*;
use screeps::{
  objects::{Creep, HasStore, RoomObjectProperties},
  ResourceType, ReturnCode,
};

use crate::role_harvester;

fn upgrade(creep: &Creep) -> bool {
  if let Some(c) = creep.room().controller() {
    let r = creep.upgrade_controller(&c);
    if r == ReturnCode::NotInRange {
      creep.memory().set("status", "Moving to Room Controller");
      creep.move_to(&c);
      return true;
    } else if r != ReturnCode::Ok {
      creep.memory().set("status", "Error upgrading controller");
      warn!("couldn't upgrade: {:?}", r);
      return false;
    }
    creep.memory().set("status", "UPGRADED!");
    true
  } else {
    creep.memory().set("status", "WTF? NO CONTROLLER?");
    warn!("creep room has no controller!");
    false
  }
}
pub fn run(creep: &Creep) -> bool {
  creep.memory().set("status", "Upgrader pending");
  if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
    upgrade(&creep)
  } else {
    creep.memory().set("status", "Harvesting");
    upgrade(&creep) || role_harvester::run(&creep)
  }
}
