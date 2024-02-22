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

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NameIn<'r> {
    pub name: &'r str,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct IdIn {
    pub id: i32,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CharacterIn {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_level: Option<i32>,
    pub is_unlocked: bool,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FoodIn {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_level: Option<i32>,
    pub is_unlocked: bool,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MachineIn {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_level: Option<i32>,
    pub is_unlocked: bool,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MissionIn {
    pub id: i32,
    pub is_unlocked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cleared: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct BusinessIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_stage_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_stage_money: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_dia: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_tables: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chef_speed_multiplier: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_speed_multiplier: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulated_customer: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulated_sales: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_characters: Option<Vec<CharacterIn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_foods: Option<Vec<FoodIn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_machines: Option<Vec<MachineIn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_missions: Option<Vec<MissionIn>>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TokenOut {
    pub token: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginInfo {
    pub id: i32,
}
