extern crate wasm_bindgen;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC : wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn browser_debug() {
  // When the `console_error_panic_hook` feature is enabled, we can call the
  // `set_panic_hook` function at least once during initialization, and then
  // we will get better error messages if our code ever panics.
  //
  // For more details see
  // https://github.com/rustwasm/console_error_panic_hook#readme
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}

extern crate nalgebra as na;

pub mod stage;
pub mod creature;
pub mod simulation;

mod api;
pub use api::*;

#[cfg(test)]
mod tests {
  use super::*;
  use simulation::*;

  #[test]
  fn sim_test() {
    let stage = Box::new(stage::SquareStage(100.));
    let mut sim = Simulation::new(stage, 123, 10);
    sim.add_behavour(Box::new(behaviours::BasicMoveBehaviour));
    sim.add_behavour(Box::new(behaviours::ScavengeBehaviour));
    sim.add_behavour(Box::new(behaviours::WanderBehaviour));
    sim.add_behavour(Box::new(behaviours::HomesickBehaviour));
    sim.add_behavour(Box::new(behaviours::StarveBehaviour));

    sim.run(5, 5);

    let mut results = Vec::new();
    println!("generations {}", sim.generations.len());
    for g in sim.generations {
      let mut creatures = Vec::new();
      for c in g.creatures {
        creatures.push(c.clone());
      }
      results.push(creatures);
    }

    // println!("{:#?}", results);
  }
}
