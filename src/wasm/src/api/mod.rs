use crate::{RunningStatistics, RunningStatisticsResults};
use wasm_bindgen::prelude::*;
use super::*;
use simulation::*;
use creature::*;

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
  foods_before_home: i32,
}

fn use_preset( sim : &mut Simulation, preset : &PresetConfig ){
  match preset.name {
    _ => {
      // default
      sim.add_behavour(Box::new(behaviours::BasicMoveBehaviour));
      sim.add_behavour(Box::new(behaviours::WanderBehaviour));
      sim.add_behavour(Box::new(behaviours::ScavengeBehaviour));
      sim.add_behavour(Box::new(behaviours::SatisfiedBehaviour));
      // sim.add_behavour(Box::new(behaviours::HomesickBehaviour));
      sim.add_behavour(Box::new(behaviours::EdgeHomeBehaviour));
      sim.add_behavour(Box::new(behaviours::StarveBehaviour));
    }
  }
}

#[derive(Serialize)]
struct GenerationStatistics {
  population: usize,
  speed : RunningStatisticsResults,
  size : RunningStatisticsResults,
  sense_range : RunningStatisticsResults,
  reach : RunningStatisticsResults,
  life_span : RunningStatisticsResults,
  age : RunningStatisticsResults,
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

  generation_statistics: Vec<GenerationStatistics>,
}

pub fn get_statistics(sim : &Simulation) -> JsValue {
  let mut population = RunningStatistics::new();
  let mut tot_speed = RunningStatistics::new();
  let mut tot_size = RunningStatistics::new();
  let mut tot_sense_range = RunningStatistics::new();
  let mut tot_reach = RunningStatistics::new();
  let mut tot_life_span = RunningStatistics::new();
  let mut tot_age = RunningStatistics::new();

  // every generation
  let generation_statistics = sim.generations.iter().map(|g| {
    let mut speed = RunningStatistics::new();
    let mut size = RunningStatistics::new();
    let mut sense_range = RunningStatistics::new();
    let mut reach = RunningStatistics::new();
    let mut life_span = RunningStatistics::new();
    let mut age = RunningStatistics::new();

    g.creatures.iter().for_each(|c|{
      speed.push(c.get_speed());
      size.push(c.get_size());
      sense_range.push(c.get_sense_range());
      reach.push(c.get_reach());
      life_span.push(c.get_life_span());
      age.push(c.age as f64);
    });

    population.push(g.creatures.len() as f64);
    tot_speed.push(speed.mean());
    tot_size.push(size.mean());
    tot_sense_range.push(sense_range.mean());
    tot_reach.push(reach.mean());
    tot_life_span.push(life_span.mean());
    tot_age.push(age.mean());

    GenerationStatistics {
      population: g.creatures.len(),
      speed: speed.as_results(),
      size: size.as_results(),
      sense_range: sense_range.as_results(),
      reach: reach.as_results(),
      life_span: life_span.as_results(),
      age: age.as_results(),
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

    generation_statistics,
  };

  JsValue::from_serde(&result).unwrap()
}

mod square_world;
pub use square_world::*;
