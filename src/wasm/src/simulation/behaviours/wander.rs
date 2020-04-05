use super::*;

// Basic behaviour for simple movement
#[derive(Debug, Copy, Clone)]
pub struct WanderBehaviour;
impl WanderBehaviour {
  fn wander(&self, creature : &mut Creature, pos : Point2<f64>){
    creature.add_objective(Objective {
      pos,
      intensity: ObjectiveIntensity::MinorCraving,
      reason: String::from("wandering"),
    });
  }
}

impl StepBehaviour for WanderBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, sim : &Simulation){
    // during orientation...
    if let Phase::ORIENT = phase {
      generation.creatures.iter_mut()
        .filter(|c| c.is_active())
        .for_each(|c| {
          let ang = sim.get_random_float(-FRAC_PI_4, FRAC_PI_4);
          let rot = na::Rotation2::new(ang);
          let target = c.get_position() + rot * c.get_direction().as_ref();
          if sim.stage.can_move_to(&target, &c) {
            self.wander(c, target);
          } else {
            self.wander(c, sim.stage.get_center());
          }
        });
    }
  }
}
