extern crate web_sys;
use web_sys::console;

pub struct Timer {
  name: String,
}

impl Timer {
  pub fn new(name: String) -> Timer {
    console::time_with_label(&name);
    Timer { name }
  }
}

impl Drop for Timer {
  fn drop(&mut self) {
    console::time_end_with_label(&self.name);
  }
}
