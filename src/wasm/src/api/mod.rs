use std::collections::HashMap;
use crate::{RunningStatistics, RunningStatisticsResults};
use wasm_bindgen::prelude::*;
use super::*;
use simulation::*;
use creature::*;
use behaviours::{BasicReproductionBehaviour, StepBehaviour};

#[derive(Deserialize)]
pub struct RandomCreatureConfig {
  pub count : usize,
  pub template : Creature,
}

#[derive(Serialize, Deserialize)]
pub struct SimulationResults {
  generations: Vec<Generation>
}

#[derive(Serialize, Deserialize)]
pub struct PresetConfig {
  name: String,
  options: HashMap<String, f64>,
}

fn primer_behaviours() -> Vec<Box<dyn StepBehaviour>>{
  vec![
    Box::new(behaviours::BasicMoveBehaviour),
    Box::new(behaviours::WanderBehaviour),
    Box::new(behaviours::CannibalismBehaviour { size_ratio: 0.8 }),
    Box::new(behaviours::ScavengeBehaviour),
    Box::new(behaviours::SatisfiedBehaviour),
    Box::new(behaviours::EdgeHomeBehaviour { disabled_edges: vec![] }),
    Box::new(behaviours::StarveBehaviour),
  ]
}

fn use_preset( sim : &mut Simulation, preset : &PresetConfig ){
  let mut behaviours = match preset.name.as_str() {
    "home_remove" => {
      let step_at_home_change = preset.options["step"] as usize;
      sim.add_generation_callback(move |sim| {
        if sim.generations.len() == step_at_home_change {
          sim.behaviours[6] = Box::new(behaviours::EdgeHomeBehaviour {
            disabled_edges: vec![0, 1, 2],
          });
        }
      });

      primer_behaviours()
    },
    _ => {
      // default
      primer_behaviours()
    }
  };

  for b in behaviours.drain(0..behaviours.len()) {
    sim.add_behavour(b);
  }
}

#[derive(Serialize)]
struct GenerationStatistics {
  population: usize,

  // traits
  speed : RunningStatisticsResults,
  size : RunningStatisticsResults,
  sense_range : RunningStatisticsResults,
  reach : RunningStatisticsResults,
  life_span : RunningStatisticsResults,

  // longevity
  age : RunningStatisticsResults,
  age_at_death : RunningStatisticsResults,
  // food related
  food_balls_available: u32,
  food_balls_eaten: u32,
  creatures_eaten: u32,

  // births/deaths
  births: u32,
  deaths: u32,
}

#[derive(Serialize)]
struct SimulationStatistics {
  num_generations: usize,

  population : RunningStatisticsResults,
  speed : RunningStatisticsResults,
  size : RunningStatisticsResults,
  sense_range : RunningStatisticsResults,
  reach : RunningStatisticsResults,
  life_span : RunningStatisticsResults,
  age : RunningStatisticsResults,
  age_at_death : RunningStatisticsResults,

  generation_statistics: Vec<GenerationStatistics>,
}

pub fn get_statistics(sim : &Simulation, species_filter : Option<String>) -> JsValue {
  let mut population = RunningStatistics::new();
  let mut tot_speed = RunningStatistics::new();
  let mut tot_size = RunningStatistics::new();
  let mut tot_sense_range = RunningStatistics::new();
  let mut tot_reach = RunningStatistics::new();
  let mut tot_life_span = RunningStatistics::new();
  let mut tot_age = RunningStatistics::new();
  let mut tot_age_at_death = RunningStatistics::new();

  // every generation
  let generation_statistics = sim.generations.iter().map(|g| {
    let mut speed = RunningStatistics::new();
    let mut size = RunningStatistics::new();
    let mut sense_range = RunningStatistics::new();
    let mut reach = RunningStatistics::new();
    let mut life_span = RunningStatistics::new();
    let mut age = RunningStatistics::new();
    let mut age_at_death = RunningStatistics::new();

    let mut food_balls_eaten = 0;
    let mut creatures_eaten = 0;
    let food_balls_available = g.food.len() as u32;

    let mut births = 0;
    let mut deaths = 0;

    let mut count = 0;

    g.creatures.iter().filter(|c| {
      species_filter.as_ref().map_or(true, |s| *s == c.species)
    }).for_each(|c|{
      count += 1;

      let t = c.get_traits();
      speed.push(t["speed"].0);
      size.push(t["size"].0);
      sense_range.push(t["sense_range"].0);
      reach.push(t["reach"].0);
      life_span.push(t["life_span"].0);
      age.push(c.age as f64);

      for eaten in &c.foods_eaten {
        match eaten.2.as_str() {
          "food_ball" => {
            food_balls_eaten += 1;
          },
          "creature" => {
            creatures_eaten += 1;
          },
          _ => {}
        }
      }

      if BasicReproductionBehaviour::will_reproduce(&c) {
        births += 1;
      }

      if !c.is_alive() {
        deaths += 1;
        age_at_death.push(c.age as f64);
        tot_age_at_death.push(c.age as f64);
      }


      tot_speed.push(t["speed"].0);
      tot_size.push(t["size"].0);
      tot_sense_range.push(t["sense_range"].0);
      tot_reach.push(t["reach"].0);
      tot_life_span.push(t["life_span"].0);
      tot_age.push(c.age as f64);
    });

    population.push(count as f64);

    GenerationStatistics {
      speed: speed.as_results(),
      size: size.as_results(),
      sense_range: sense_range.as_results(),
      reach: reach.as_results(),
      life_span: life_span.as_results(),

      population: count,
      age: age.as_results(),
      age_at_death: age_at_death.as_results(),

      food_balls_available,
      food_balls_eaten,
      creatures_eaten,

      births,
      deaths,
    }
  }).collect();

  let result = SimulationStatistics {
    num_generations: sim.generations.len(),

    population: population.as_results(),
    speed: tot_speed.as_results(),
    size: tot_size.as_results(),
    sense_range: tot_sense_range.as_results(),
    reach: tot_reach.as_results(),
    life_span: tot_life_span.as_results(),
    age: tot_age.as_results(),
    age_at_death: tot_age_at_death.as_results(),

    generation_statistics,
  };

  JsValue::from_serde(&result).unwrap()
}

mod square_world;
pub use square_world::*;
