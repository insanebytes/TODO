use jiff::civil::DateTime;
use serde::{Deserialize,Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub date: DateTime,
    pub done: bool,
}