use linera_sdk::base::Owner;
use linera_views::{context::Context, map_view::MapView, views::View};
use serde::{Deserialize, Serialize};
use linera_views::views::{ViewError, View};

#[derive(Debug, Serialize, Deserialize, View)]
pub struct RadRunScores<C> {
    pub scores: MapView<C, Owner, u64>,
}

impl<C> RadRunScores<C> {
    pub fn new(context: C) -> Self {
        Self {
            scores: MapView::new(context),
        }
    }
}

pub type RadRunScoresState = RadRunScores<Context>;
