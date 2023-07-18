use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TruthValue {
    pub strength: f32,
    pub confidence: f32,
}
