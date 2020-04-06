use super::*;

// Primer behaviour. Go home at 2 food
#[derive(Debug, Copy, Clone)]
pub struct SatisfiedBehaviour;
impl SatisfiedBehaviour {
  fn how_homesick(&self, creature : &Creature) -> Option<Objective> {

    match creature.foods_eaten.len() {
      // if no food... keep going
      x if x == 0 => None,
      // if more than 1 foods... go home
      x if x > 1 => Some(Objective {
        pos: creature.home_pos,
        intensity: ObjectiveIntensity::MajorCraving,
        reason: String::from("satisfied"),
      }),
      _ => HomesickBehaviour::how_homesick(&creature),
    }
  }
}

impl StepBehaviour for SatisfiedBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, _sim : &Simulation){
    // during orientation...
    if let Phase::ORIENT = phase {
      generation.creatures.iter_mut()
        .filter(|c| c.is_active())
        .filter_map(|c|
          self.how_homesick(c)
            .map(|i| (c, i))
        )
        .for_each(|(c, o)| {
          c.add_objective(o);

          if c.can_reach(&c.home_pos) {
            c.sleep();
          }
        });
    }
  }
}
