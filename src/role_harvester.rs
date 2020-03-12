use log::*;
use screeps::{
  find,
  objects::{Creep, HasPosition, HasStore, RoomObjectProperties, Structure},
  ResourceType, ReturnCode,
};

fn is_target(s: &Structure) -> bool {
  match s {
    Structure::Extension(s) => {
      s.store_free_capacity(Some(ResourceType::Energy)) > 0
    }
    Structure::Spawn(s) => {
      s.store_free_capacity(Some(ResourceType::Energy)) > 0
    }
    Structure::Tower(s) => {
      s.store_free_capacity(Some(ResourceType::Energy)) > 0
    }
    _ => false,
  }
}

pub fn run(creep: &Creep) -> bool {
  creep.memory().set("status", "Harvester pending");
  if creep.memory().bool("harvesting")
    && creep.store_free_capacity(Some(ResourceType::Energy)) == 0
  {
    // Creep is full
    creep.memory().set("harvesting", false);
  } else if creep.store_of(ResourceType::Energy) == 0 {
    creep.memory().set("harvesting", true);
  }

  if creep.memory().bool("harvesting") {
    // Creep needs energy
    let source = &creep.room().find(find::SOURCES)[0];
    if creep.pos().is_near_to(source) {
      let r = creep.harvest(source);
      if r != ReturnCode::Ok {
        creep.memory().set("status", "Error harvesting");
        warn!("couldn't harvest: {:?}", r);
        false
      } else {
        creep.memory().set("status", "Harvesting node");
        true
      }
    } else {
      creep.memory().set("status", "Moving to node");
      creep.move_to(source);
      true
    }
  } else {
    // Creep needs to drop off energy
    let targets: Vec<Structure> = creep.room().find(find::STRUCTURES);
    let targets: Vec<&Structure> =
      targets.iter().filter(|s| is_target(s)).collect::<Vec<&Structure>>();
    if targets.len() > 0 {
      let target: &Structure = targets[0];
      if creep.pos().is_near_to(target) {
        if let Some(target) = target.as_transferable() {
          creep.memory().set("status", "Transfering Energy");
          let r = creep.transfer_all(target, ResourceType::Energy);
          if r != ReturnCode::Ok {
            creep.memory().set("status", "Error transfering energy");
            warn!("Couldn't transfer energy: {:?}", r);
            return false;
          }
          true
        } else {
          creep.memory().set("status", "Error: cannot transfer to target");
          warn!("Target is not transferable");
          false
        }
      } else {
        creep.memory().set("status", "Moving to drop off energy");
        creep.move_to(target);
        true
      }
    } else {
      creep.memory().set("status", "ENERGY FULL");
      false
    }
  }
}
