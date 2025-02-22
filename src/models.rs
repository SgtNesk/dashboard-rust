use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub hours_attended: f32,
}
