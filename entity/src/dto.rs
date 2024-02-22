use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Character {
    pub name: String,
    pub current_level: i32,
    pub is_unlocked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Food {
    pub name: String,
    pub current_level: i32,
    pub is_unlocked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Machine {
    pub name: String,
    pub current_level: i32,
    pub is_unlocked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Mission {
    pub mission_index: i32,
    pub is_unlocked: bool,
    pub is_cleared: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Business {
    pub current_stage_number: i32,
    pub current_stage_money: i32,
    pub current_dia: i32,
    pub enabled_tables: i32,
    pub chef_speed_multiplier: f32,
    pub server_speed_multiplier: f32,
    pub current_characters: Vec<Character>,
    pub current_foods: Vec<Food>,
    pub current_machines: Vec<Machine>,
    pub current_missions: Vec<Mission>,
}
