// use screeps::objects::{HasPosition, RoomObjectProperties};
// use screeps::pathfinder;

// pub fn find_nearest<'a, S, T>(src: &S, targets: Vec<'a, T>) -> Option<'a, T>
// where
//   S: HasPosition,
//   T: RoomObjectProperties,
// {
//   if targets.len() == 0 {
//     return None;
//   }
//   let mut nearest = &targets[0];
//   let mut nearest_cost = 9999;
//   let range = 9999;
//
//   for target in targets {
//     let opts = pathfinder::SearchOptions::new();
//     let results = pathfinder::search(src, &target, range, opts);
//     if !results.incomplete && results.cost < nearest_cost {
//       nearest = &target;
//       nearest_cost = results.cost;
//     }
//   }
//   Some(*nearest)
// }
