use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hanzi {
    pub id: String,
    pub children: Option<Vec<String>>,
    pub result: Option<Vec<String>>
}