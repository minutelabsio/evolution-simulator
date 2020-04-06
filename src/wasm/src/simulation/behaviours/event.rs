use super::*;

#[derive(Debug, Clone)]
pub struct EventBehaviour<F>(pub usize, pub F)
where F : Fn(&mut Generation) -> ();


impl<F> StepBehaviour for EventBehaviour<F>
where F : Fn(&mut Generation) -> () {
  fn apply(&self, phase : Phase, generation : &mut Generation, sim : &Simulation){
    if let Phase::PRE = phase {
      if sim.generations.len() == self.0 {
        self.1(generation);
      }
    }
  }
}
