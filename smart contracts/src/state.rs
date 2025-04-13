use linera_sdk::base::Owner;
use linera_views::{common::Context, views::View, views::MapView};
use serde::{Deserialize, Serialize};
use linera_views::views::{ViewError, View};

#[derive(Debug, Serialize, Deserialize, View)]
pub struct RadRunScores {
    pub scores: MapView<Owner, u64>,
}

impl RadRunScores {
    pub fn new(context: &Context) -> Self {
        Self {
            scores: MapView::new(context.clone()),
        }
    }
}

pub type RadRunScoresState = RadRunScores;
