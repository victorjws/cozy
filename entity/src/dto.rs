use crate::character::Model as CharacterModel;
use crate::food::Model as FoodModel;
use crate::machine::Model as MachineModel;
use crate::mission::Model as MissionModel;
use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub current_level: i32,
    pub is_unlocked: bool,
}

impl From<CharacterModel> for Character {
    fn from(item: CharacterModel) -> Self {
        Self {
            id: item.id,
            name: item.name,
            current_level: 0,
            is_unlocked: false,
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub current_level: i32,
    pub is_unlocked: bool,
}

impl From<FoodModel> for Food {
    fn from(item: FoodModel) -> Self {
        Self {
            id: item.id,
            name: item.name,
            current_level: 0,
            is_unlocked: false,
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Machine {
    pub id: i32,
    pub name: String,
    pub current_level: i32,
    pub is_unlocked: bool,
}

impl From<MachineModel> for Machine {
    fn from(item: MachineModel) -> Self {
        Self {
            id: item.id,
            name: item.name,
            current_level: 0,
            is_unlocked: false,
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Mission {
    pub id: i32,
    pub is_unlocked: bool,
    pub is_cleared: bool,
}

impl From<MissionModel> for Mission {
    fn from(item: MissionModel) -> Self {
        Self {
            id: item.id,
            is_unlocked: false,
            is_cleared: false,
        }
    }
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
    pub accumulated_customer: i32,
    pub accumulated_sales: i32,
    pub current_characters: Vec<Character>,
    pub current_foods: Vec<Food>,
    pub current_machines: Vec<Machine>,
    pub current_missions: Vec<Mission>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Base<'r> {
    pub name: &'r str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OnlyId {
    pub id: i32,
}
