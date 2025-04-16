use linera_views::views::RootView;
use linera_base::owner::Owner;
use linera_sdk::views::{RootView, ViewStorageContext, MapView};
use linera_sdk::views::View; // for .load(), .save(), etc.

#[derive(RootView)]
#[view(context = "ViewStorageContext")]
pub struct RadRunScores {
    pub scores: MapView<Owner, u64>,
}
