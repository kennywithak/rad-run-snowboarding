use serde::{Deserialize, Serialize};

pub mod interface;
pub mod state;

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    SetScore(u64),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {}

#[derive(Debug, Deserialize, Serialize)]
pub enum Query {
    GetTopScores(u32),
}

pub use crate::state::RadRunScores;
pub use crate::state::RadRunScoresState;
