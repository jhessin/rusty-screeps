use screeps::objects::*;

/// The creep state holds the state of a creep.
/// What it is doing, where it is going.
/// It should be automatically saved to the creep's memory.
pub trait CreepState {
  fn set_status(&self, msg: &str);
  fn set_state(&self, state: State);
  fn get_state(&self) -> State;
  fn run_state(&self, state: State) -> bool;
}

pub enum State {
  Harvesting(String),
  Delivering(String),
  Withdrawing(String),
  Building(String),
  Repairing(String),
  Dismantling(String),
  Attacking(String),
  AttackingController(String),
  Reserving(String),
  Claiming(String),
  Healing(String),
  PickingUp(String),
  Unassigned,
}

impl CreepState for Creep {
  fn set_status(&self, msg: &str) {
    self.memory().set("status", &msg);
  }

  fn set_state(&self, state: State) {
    use State::*;
    match state {
      Harvesting(id) => {
        self.memory().set("state", "Harvesting");
        self.memory().set("target", id);
      }
      Delivering(id) => {
        self.memory().set("state", "Delivering");
        self.memory().set("target", id);
      }
      Withdrawing(id) => {
        self.memory().set("state", "Withdrawing");
        self.memory().set("target", id);
      }
      Building(id) => {
        self.memory().set("state", "Building");
        self.memory().set("target", id);
      }
      Repairing(id) => {
        self.memory().set("state", "Repairing");
        self.memory().set("target", id);
      }
      Dismantling(id) => {
        self.memory().set("state", "Dismantling");
        self.memory().set("target", id);
      }
      Attacking(id) => {
        self.memory().set("state", "Attacking");
        self.memory().set("target", id);
      }
      AttackingController(id) => {
        self.memory().set("state", "AttackingController");
        self.memory().set("target", id);
      }
      Reserving(id) => {
        self.memory().set("state", "Reserving");
        self.memory().set("target", id);
      }
      Claiming(id) => {
        self.memory().set("state", "Claiming");
        self.memory().set("target", id);
      }
      Healing(id) => {
        self.memory().set("state", "Healing");
        self.memory().set("target", id);
      }
      PickingUp(id) => {
        self.memory().set("state", "PickingUp");
        self.memory().set("target", id);
      }
      Unassigned => {
        self.memory().set("state", "Unassigned");
        self.memory().set("target", "");
      }
    }
  }

  fn get_state(&self) -> State {
    let state =
      self.memory().string("state").unwrap_or_default().unwrap_or_default();
    let target =
      self.memory().string("target").unwrap_or_default().unwrap_or_default();

    match (&state[..], &target[..]) {
      ("Harvesting", id) => State::Harvesting(id.to_string()),
      ("Delivering", id) => State::Delivering(id.to_string()),
      ("Withdrawing", id) => State::Withdrawing(id.to_string()),
      ("Building", id) => State::Building(id.to_string()),
      ("Repairing", id) => State::Repairing(id.to_string()),
      ("Dismantling", id) => State::Dismantling(id.to_string()),
      ("Attacking", id) => State::Attacking(id.to_string()),
      ("AttackingController", id) => State::AttackingController(id.to_string()),
      ("Reserving", id) => State::Reserving(id.to_string()),
      ("Claiming", id) => State::Claiming(id.to_string()),
      ("Healing", id) => State::Healing(id.to_string()),
      ("PickingUp", id) => State::PickingUp(id.to_string()),
      _ => State::Unassigned,
    }
  }

  fn run_state(&self, state: State) -> bool {
    match state {
      State::Harvesting(_node) => true,
      _ => false,
    }
  }
}
