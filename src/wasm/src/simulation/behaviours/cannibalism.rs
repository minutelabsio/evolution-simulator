use super::*;

// Bigger creatures eat smaller ones
pub struct CannibalismBehaviour {
  pub size_ratio : f64, // size ratio for which larger blobs eat smaller blobs
}

impl CannibalismBehaviour {
  fn for_pred_prey_pair(&self, creatures: &mut Vec<Creature>, func: &mut dyn FnMut(&mut Creature, &mut Creature)) {

    for i in 1..creatures.len(){
      if !creatures[i].is_active(){ continue; }
      let (before, after) = creatures.split_at_mut(i);
      after.iter_mut().filter(|c| c.is_active()).for_each(|prey| {
        let predator = &mut before[i - 1];
        let (predator, prey) = if predator.get_size() > prey.get_size() {
          (predator, prey)
        } else {
          (prey, predator)
        };

        // some can get killed during this loop... so check again
        if !predator.is_active() || !prey.is_active() { return }
        if predator.get_size() * self.size_ratio < prey.get_size() { return }

        func(predator, prey);
      });
    }
  }
}

impl StepBehaviour for CannibalismBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, sim : &Simulation){
    // Chase...
    if let Phase::ORIENT = phase {
      use std::collections::HashMap;
      let mut target_prey_speed = HashMap::new();

      self.for_pred_prey_pair(&mut generation.creatures, &mut |predator, prey| {
        if prey.within_flee_distance(&predator.get_position()) {
          let ang = sim.get_random_float(-FRAC_PI_4, FRAC_PI_4);
          let rot = na::Rotation2::new(ang);
          let dir = predator.get_position() - prey.get_position();
          // this is roughly the position of the predator, but a bit fuzzy
          // to add an element of randomness
          let noisy = prey.get_position() + rot * dir;
          prey.add_objective(Objective {
            pos: noisy,
            intensity: ObjectiveIntensity::VitalAversion,
            reason: String::from("running away"),
          });
        }

        if !predator.can_see(&prey.get_position()) {
          // predator can't see prey
          return;
        }

        if let Some(prev_speed) = target_prey_speed.get(&predator.id) {
          if *prev_speed < prey.get_speed() {
            // seeing many, so favour the one that's slower
            return;
          }
        }

        target_prey_speed.insert(predator.id, prey.get_speed());

        let intensity = match predator.foods_eaten.len() {
          0 => ObjectiveIntensity::VitalCraving,
          1 => ObjectiveIntensity::ModerateCraving,
          _ => ObjectiveIntensity::MinorCraving,
        };

        predator.add_objective(
          Objective {
            pos: prey.get_position(),
            intensity,
            reason: String::from("see prey"),
          });
      });
    }

    // Eat...
    if let Phase::ACT = phase {
      let steps = generation.steps;
      self.for_pred_prey_pair(&mut generation.creatures, &mut |predator, prey| {
        if !predator.can_reach(&prey.get_position()) { return }

        // now we can canibalize
        predator.eat_food(steps, prey);
        prey.kill();
      });
    }
  }
}
