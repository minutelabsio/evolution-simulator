use super::*;

// starve if no food
#[derive(Debug, Copy, Clone)]
pub struct StarveBehaviour;
impl StarveBehaviour {
  fn check_starvation(&self, creature : &mut Creature){
    if creature.foods_eaten.len() < 1 {
      creature.kill();
    }
  }
}

impl StepBehaviour for StarveBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, _sim : &Simulation){
    if let Phase::INIT = phase {
      // if the creature has a speed of zero, it starves. period.
      generation.creatures.iter_mut()
        .filter(|c| {
          c.get_speed() == 0.
        })
        .for_each(|c| c.kill());
    }

    if let Phase::POST = phase {
      let food_available = generation.get_available_food().len();
      if food_available == 0 {
        // if there is no food left after this step, check for starvation
        generation.creatures.iter_mut()
          .filter(|c| {
            c.is_active()
          })
          .for_each(|c| self.check_starvation(c));
      }
    }

    if let Phase::FINAL = phase {
      generation.creatures.iter_mut()
        .filter(|c| {
          c.is_alive()
        })
        .for_each(|c| self.check_starvation(c));
    }
  }
}
