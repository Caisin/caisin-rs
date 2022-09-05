use serde::{Deserialize, Serialize};
use crate::models::Filed;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub comment: String,
    pub fields: Option<Vec<Filed>>,
}

