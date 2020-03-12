use screeps::{
  find,
  objects::{HasPosition, RoomObjectProperties, StructureTower},
};

pub fn run(tower: &StructureTower) {
  let closest_hostile = tower.pos().find_closest_by_range(find::HOSTILE_CREEPS);
  let closest_damaged_structure = tower.room().find(find::STRUCTURES);
  let closest_damaged_structure = closest_damaged_structure
    .iter()
    .filter(|structure| {
      structure
        .as_attackable()
        .map(|s| s.hits() < s.hits_max())
        .unwrap_or(false)
    })
    .min_by_key(|structure| tower.pos().get_range_to(*structure));
  if let Some(closest_hostile) = closest_hostile {
    tower.attack(&closest_hostile);
  } else if let Some(closest_damaged_structure) = closest_damaged_structure {
    tower.repair(closest_damaged_structure);
  }
}
