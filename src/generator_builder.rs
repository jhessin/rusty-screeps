use log::*;
use screeps::{
  objects::{HasStore, StructureSpawn},
  Part, ReturnCode,
};

fn get_body(spawn: &StructureSpawn) -> Option<Vec<Part>> {
  use Part::*;
  let options = vec![
    vec![Work, Work, Carry, Carry, Move, Move, Move, Move],
    vec![Work, Work, Carry, Move, Move, Move, Move],
    vec![Work, Work, Carry, Move, Move, Move],
    vec![Work, Carry, Move, Move, Move],
    vec![Work, Carry, Move, Move],
    vec![Work, Carry, Move],
  ];
  for body in options.iter() {
    if spawn.energy() >= body.iter().map(|p| p.cost()).sum() {
      return Some(body.to_vec());
    }
  }
  None
}

pub fn run(spawner: &StructureSpawn) {
  if let Some(body) = get_body(&spawner) {
    let name = format!("Builder{}", screeps::game::time());
    let res = spawner.spawn_creep(&body, &name);

    if res != ReturnCode::Ok {
      warn!("couldn't spawn: {:?}", res);
    }
  }
}
