use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Student {
    // for ignore the id field on creating time
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
}