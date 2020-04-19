use super::*;

// search for food
#[derive(Debug, Copy, Clone)]
pub struct ScavengeBehaviour;
impl ScavengeBehaviour {
  fn is_creature_hungry(creature : &Creature) -> bool {
    creature.is_active() && creature.foods_eaten.len() < 2
  }

  fn look_for_food(&self, creature : &mut Creature, available_food : &Vec<Food>){
    if let Some(food) = self.nearest_food(creature, available_food) {
      if creature.can_see(&food.position) {

        // how hungry is it?
        let intensity = match creature.foods_eaten.len() {
          0 => ObjectiveIntensity::VitalCraving,
          1 => ObjectiveIntensity::ModerateCraving,
          _ => ObjectiveIntensity::MinorCraving,
        };

        creature.add_objective(Objective {
          pos: food.position,
          intensity,
          reason: String::from("see food"),
        });
      }
    }
  }

  fn try_find_food(&self, creature : &Creature, available_food : &Vec<Food>) -> Option<Food> {
    if let Some(food) = self.nearest_food(creature, available_food) {
      if !food.is_eaten() && creature.can_reach(&food.position) {
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
        .filter(|c| Self::is_creature_hungry(&c))
        .for_each(|c| self.look_for_food(c, &available_food));
    }

    // when it is able to interact
    if let Phase::ACT = phase {
      let mut available_food = generation.get_available_food();

      for index in 0..generation.creatures.len() {
        let creature = &mut generation.creatures[index];
        if Self::is_creature_hungry(&creature) {
          if let Some(food) = self.try_find_food(creature, &available_food) {
            creature.eat_food(generation.steps, &food);
            generation.mark_food_eaten(&food);
            let index = available_food.iter().position(|f| *f == food).unwrap();
            available_food.remove(index);
          }
        }
      }
    }
  }
}
