use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Request {
    Read { key: String },
    Write { key: String, value: String },
    Delete { key: String },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Response {
    Success(Option<String>),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CompactionStrategy {
   SizeTiered,
   LevelBased, 
}