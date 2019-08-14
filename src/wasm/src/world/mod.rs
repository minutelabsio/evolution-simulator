use crate::creature::*;
extern crate rand;
use rand::{SeedableRng, Rng, rngs::SmallRng};

pub enum Phase<'a> {
  Tick,
  Interact(&'a Box<Creature>),
}

pub struct World {
  // how big is it in one dimension
  pub size : u32,

  pub generations : Vec<Box<Generation>>,
}

impl World {
  pub fn new( size: u32 ) -> Self {

    let generations = Vec::new();

    World {
      size,
      generations,
    }
  }

  pub fn run(&mut self, creature_count: u32, max_generations : u32){
    let mut generation = Box::new(Generation::new(&self, creature_count));
    let mut keep_going = generation.has_living_creatures();

    for _gen in 1..max_generations {
      if !keep_going { break; }

      let next = Box::new(Generation::from_previous( &generation ));
      self.generations.push(generation);
      generation = next;
      keep_going = generation.has_living_creatures();
    }

    self.generations.push(generation);
  }
}

pub struct Generation {
  creatures: Vec<Box<Creature>>,
}

impl Generation {
  pub fn new( world : &World, creature_count : u32 ) -> Self {
    // prepare a deterministic generator:
    let mut rng = SmallRng::seed_from_u64(123);

    let creatures = (0..creature_count).map(|_n| {
      // random creature starting position
      // TODO: original started creatures on edges
      let x = rng.gen_range(0., world.size as f64);
      let y = rng.gen_range(0., world.size as f64);

      Box::new(Creature::new( x, y ))
    }).collect();

    let mut gen = Generation {
      creatures,
    };

    gen.step_to_completion();

    gen
  }

  pub fn from_previous( previous : &Generation ) -> Self {
    unimplemented!()
  }

  pub fn has_living_creatures(&self) -> bool {
    self.creatures.iter().any(|c| c.is_alive())
  }

  // advance the generation to its end
  fn step_to_completion(&mut self) {
    loop {
      self.step();

      if !self.creatures.iter().any(|c| c.is_active()) {
        // break when all asleep or dead
        break;
      }
    }
  }

  fn step(&mut self){
    // tick phase
    self.creatures.iter_mut().for_each(|c|{
      (*c).apply_phase( &Phase::Tick );
    });

    // interaction phase
    // check interaction with other creatures
    for i in 0..self.creatures.len() {
      let mut c = self.creatures.remove(i);
      self.creatures.iter().for_each(|other| {
        (*c).apply_phase( &Phase::Interact(&other) );
      });
      self.creatures.insert(i, c);
    }
  }
}
