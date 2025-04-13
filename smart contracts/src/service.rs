use linera_sdk::service::*;
use linera_base::abi::{ServiceAbi, WithServiceAbi};
use linera_views::views::ViewError;
use rad_run_scores::RadRunScores;

use crate::state::RadRunScores;

#[derive(Default, WithServiceAbi)]
pub struct RadRunScoresService {
    pub state: RadRunScores,
}

impl Service for RadRunScoresService {
    type Error = ViewError;
    type Storage = RadRunScores;
    type Message = ();
    type State = RadRunScores;
    type Query = ();

    async fn handle_query(
        &self,
        _query: Self::Query,
    ) -> Result<Vec<Message>, Self::Error> {
        Ok(vec![])
    }
}

impl ServiceAbi for RadRunScoresService {
    type Query = rad_run_scores::Operation;
    type QueryResponse = u64;
}

#[async_trait::async_trait]
impl Service for RadRunScoresService {
    async fn handle_query(
        &self,
        query: Self::Query,
    ) -> Result<Self::QueryResponse, ViewError> {
        match query {
            rad_run_scores::Operation::GetScore { owner } => {
                Ok(self.state.scores.get(&owner)?.unwrap_or(0))
            }
        }
    }
}

fn main() {}
