use super::*;
use std::f64::consts::FRAC_PI_4;

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

// Basic behaviour for returning home before it will die
#[derive(Debug, Copy, Clone)]
pub struct HomesickBehaviour;
impl HomesickBehaviour {
  fn how_homesick(&self, creature : &Creature) -> Option<ObjectiveIntensity> {
    let dist = (creature.home_pos - creature.get_position()).norm();
    let cost = creature.get_motion_energy_cost();
    let steps_to_home = dist / creature.speed;
    let homesick_factor = creature.energy / cost - steps_to_home;

    match homesick_factor {
      // pleanty of energy... can stay out longer
      x if x > 10.0 => None,
      // ok starting to miss home
      x if x > 5.0 => Some(ObjectiveIntensity::ModerateCraving),
      x if x > 0.0 => Some(ObjectiveIntensity::MajorCraving),
      _ => Some(ObjectiveIntensity::VitalCraving),
    }
  }
}

impl StepBehaviour for HomesickBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, sim : &Simulation){
    // during orientation...
    if let Phase::ORIENT = phase {
      generation.creatures.iter_mut()
        .filter(|c| c.is_alive())
        .filter_map(|c|
          self.how_homesick(c)
            .map(|i| (c, i))
        )
        .for_each(|(c, i)| {
          c.add_objective(c.home_pos, i);
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
  fn look_for_food(&self, creature : &mut Creature, available_food : &Vec<Food>){
    if let Some(food) = self.nearest_food(creature, available_food) {
      if creature.can_see(&food.position) {

        // how hungry is it?
        let intensity = match creature.foods_eaten {
          0 => ObjectiveIntensity::MajorCraving,
          1 => ObjectiveIntensity::ModerateCraving,
          _ => ObjectiveIntensity::MinorCraving,
        };

        creature.add_objective(food.position, intensity);
      }
    }
  }

  fn try_find_food(&self, creature : &mut Creature, available_food : &Vec<Food>) -> Option<Food> {
    if let Some(food) = self.nearest_food(creature, available_food) {
      if creature.can_reach(&food.position) {
        return Some(food);
      }
    }

    None
  }

  fn nearest_food(&self, creature : &Creature, available_food : &Vec<Food>) -> Option<Food> {
    let pos = creature.get_position();
    let nearest = available_food.iter()
      .map(|f| (f.position - pos).norm())
      .filter(|n| !n.is_nan())
      .enumerate()
      .min_by(|a, b| (a.1).partial_cmp(&b.1).unwrap());

      if let Some((index, _dist)) = nearest {
        Some(available_food[index].clone())
      } else {
        None
      }
  }
}

impl StepBehaviour for ScavengeBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, _sim : &Simulation){
    if let Phase::ORIENT = phase {
      let available_food = generation.get_available_food();

      generation.creatures.iter_mut()
        .filter(|c| c.is_alive())
        .for_each(|c| self.look_for_food(c, &available_food));
    }

    // when it is able to interact
    if let Phase::ACT = phase {
      let available_food = generation.get_available_food();

      let food_results : Vec<Option<Food>> = generation.creatures.iter_mut()
        .map(|c| {
          if c.is_alive() {
            self.try_find_food(c, &available_food)
          } else {
            None
          }
        })
        .collect();

      for index in 0..generation.creatures.len() {
        if let Some(food) = &food_results[index] {
          generation.creatures[index].eat_food();
          generation.mark_food_eaten(&food);
        }
      }
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
