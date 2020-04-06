use super::*;
use std::cell::{RefMut};
use rand::{rngs::SmallRng};

// Basic behaviour for simple movement
#[derive(Debug, Copy, Clone)]
pub struct BasicReproductionBehaviour;

impl BasicReproductionBehaviour {
  fn will_reproduce(&self, creature : &Creature) -> bool {
    creature.foods_eaten.len() > 1
  }

  fn reproduce(&self, creature : &Creature, rng : &mut RefMut<SmallRng>) -> Vec<Creature> {
    if self.will_reproduce(&creature) {
      vec![creature.mutate(rng)]
    } else {
      vec![]
    }
  }
}

impl ReproductionBehaviour for BasicReproductionBehaviour {
  fn reproduce(&self, creatures : &Vec<Creature>, sim : &Simulation) -> Vec<Creature> {
    creatures.iter().filter(|c|
      c.is_alive()
    ).flat_map(|c| {
      let mut rng = sim.rng.borrow_mut();
      let mut ctrs = self.reproduce(&c, &mut rng);
      let grown = c.grow_older();
      ctrs.push(grown);
      ctrs
    }).collect()
  }
}
