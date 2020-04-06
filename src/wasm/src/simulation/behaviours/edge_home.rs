use crate::math::*;
use super::*;

// set the creature's home position to be the nearest point on the nearest edge
#[derive(Debug, Clone)]
pub struct EdgeHomeBehaviour {
  pub disabled_edges: Vec<usize>,
}

impl EdgeHomeBehaviour {
  fn set_home(&self, creature : &mut Creature, sim : &Simulation){
    let edges = sim.stage.get_edges();
    let nearest_edge = edges.iter().enumerate()
      .filter(|(i, _)| !self.disabled_edges.contains(i))
      .filter_map(|(_i, e)| {
        distance_to_line(&e.0, &e.1, &creature.pos).map(|d| (e, d))
      })
      .min_by(|a, b| {
        a.1.partial_cmp(&b.1).unwrap()
      });

    if let Some((edge, _)) = nearest_edge {
      let home = edge.0 + project_to_line(&edge.0, &edge.1, &creature.pos).unwrap();
      creature.home_pos = home;
    } else {
      // no near edge... set to infinity... goodbye
      creature.home_pos = Point2::new(std::f64::MAX, 0.);
    }
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
