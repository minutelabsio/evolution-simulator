use uuid::Uuid;
use na::Point2;
use super::{Edible, FoodType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FoodStatus {
  Available,
  Eaten(usize), // step the food was eaten at
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Food {
  pub id : Uuid,
  pub position: Point2<f64>,
  pub status: FoodStatus,
}
impl Food {
  pub fn new(position: Point2<f64>) -> Self {
    Self {
      id: Uuid::new_v4(),
      position,
      status: FoodStatus::Available
    }
  }
  pub fn is_eaten(&self) -> bool { self.status != FoodStatus::Available }
}

impl Edible for Food {
  fn get_edible_id(&self) -> Uuid { self.id }
  fn get_type(&self) -> FoodType { "food_ball".into() }
}