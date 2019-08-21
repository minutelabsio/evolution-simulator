use super::*;
use std::f64::consts::FRAC_PI_4;

const MOTION_ENERGY_COST : f64 = 0.01;

// Basic behaviour for simple movement
#[derive(Debug, Copy, Clone)]
pub struct WanderBehaviour;
impl WanderBehaviour {
  fn wander(&self, creature : &mut Creature, target : Point2<f64>){
    creature.add_objective(target, ObjectiveIntensity::MinorCraving);
  }
}

impl StepBehaviour for WanderBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, sim : &Simulation){
    // during orientation...
    if let Phase::ORIENT = phase {
      generation.creatures.iter_mut()
        .filter(|c| c.is_alive())
        .for_each(|c| {
          let ang = sim.get_random_float(-FRAC_PI_4, FRAC_PI_4);
          let rot = na::Rotation2::new(ang);
          let randomPos = c.get_position() + rot * c.get_direction().as_ref();
          self.wander(c, randomPos);
        });
    }
  }
}

// Basic behaviour for simple movement
#[derive(Debug, Copy, Clone)]
pub struct BasicMoveBehaviour;
impl BasicMoveBehaviour {
  fn move_creature(&self, creature : &mut Creature){
    // move
    let new_pos = creature.get_position() + creature.speed * creature.get_direction().as_ref();
    creature.move_to( new_pos );
    // energy cost
    let last = creature.get_last_position().expect("Can not get last position.");
    let displacement = creature.get_position() - last;
    creature.apply_energy_cost( MOTION_ENERGY_COST * displacement.norm() )
  }
}

impl StepBehaviour for BasicMoveBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, _sim : &Simulation){
    if let Phase::MOVE = phase {
      generation.creatures.iter_mut()
        .filter(|c| c.is_alive())
        .for_each(|c| self.move_creature(c));
    }
  }
}

// search for food
#[derive(Debug, Copy, Clone)]
pub struct ScavengeBehaviour;
impl ScavengeBehaviour {
  fn look_for_food(&self, creature : &mut Creature, available_food : &Vec<Point2<f64>>){
    if let Some(food_index) = self.nearest_food(creature, available_food) {
      let pos = available_food[food_index];
      creature.add_objective(pos, ObjectiveIntensity::ModerateCraving);
    }
  }

  fn nearest_food(&self, creature : &Creature, available_food : &Vec<Point2<f64>>) -> Option<usize> {
    let pos = creature.get_position();
    let nearest = available_food.iter()
      .map(|food_pos| (food_pos - pos).norm())
      .filter(|n| !n.is_nan())
      .enumerate()
      .min_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());

    if let Some((index, dist)) = nearest {
      if dist <= creature.sense_range {
        Some(index)
      } else {
        None
      }
    } else {
      None
    }
  }
}

impl StepBehaviour for ScavengeBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, _sim : &Simulation){
    if let Phase::ORIENT = phase {
      let available_food = generation.food.iter().filter(|(.., status)| {
        if let FoodStatus::Available = status { true } else { false }
      }).map(|(pos, ..)| *pos).collect();

      generation.creatures.iter_mut()
        .filter(|c| c.is_alive())
        .for_each(|c| self.look_for_food(c, &available_food));
    }
  }
}

// // Bigger creatures eat smaller ones
// pub struct CannibalismBehaviour {
//   // controls nutritional value of a grotesque meal
//   pub multiplier: f64,
//   // within this radius, one can eat another
//   pub interaction_range: f64,
// }
//
// impl StepBehaviour for CannibalismBehaviour {
//   fn apply(&mut self, phase : Phase, generation : &mut generation, stage : &Simulation){
//     if let Phase::ACT = phase {
//       generation.creatures.iter().copied()
//         .filter(|c| c.is_alive())
//         .for_each(|c| );
//     }
//   }
// }
