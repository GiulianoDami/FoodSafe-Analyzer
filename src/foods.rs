use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodItem {
    pub name: String,
    pub cooking_methods: Vec<String>,
    pub pah_risk_level: u8, // 0-100 scale
}

impl FoodItem {
    pub fn new(name: &str, cooking_methods: Vec<String>, pah_risk_level: u8) -> Self {
        FoodItem {
            name: name.to_string(),
            cooking_methods,
            pah_risk_level,
        }
    }
}

pub fn check_pah_risk(food_item: &FoodItem, cooking_method: &str) -> bool {
    food_item.cooking_methods.contains(&cooking_method.to_string())
}