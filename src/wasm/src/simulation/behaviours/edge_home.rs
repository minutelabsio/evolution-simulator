use super::*;

// set the creature's home position to be the nearest point on the nearest edge
#[derive(Debug, Copy, Clone)]
pub struct EdgeHomeBehaviour;
impl EdgeHomeBehaviour {
  fn set_home(&self, creature : &mut Creature, sim : &Simulation){
    let home = sim.stage.get_nearest_edge_point(&creature.pos);
    creature.home_pos = home;
  }
}

impl StepBehaviour for EdgeHomeBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, sim : &Simulation){
    if let Phase::PRE = phase {
      generation.creatures.iter_mut()
        .filter(|c| c.is_alive())
        .for_each(|c| self.set_home(c, sim));
    }
  }
}
