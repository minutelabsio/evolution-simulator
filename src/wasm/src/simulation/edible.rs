use uuid::Uuid;
pub type FoodType = String;

pub trait Edible {
  fn get_edible_id(&self) -> Uuid;
  fn get_type(&self) -> FoodType;
}