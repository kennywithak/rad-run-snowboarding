use linera_sdk::base::Owner;
use linera_sdk::contract::*;
use linera_views::views::ViewError;

use crate::state::RadRunScores;

#[derive(Default)]
pub struct RadRunScoresContract {
    pub state: RadRunScores,
}

#[async_trait]
impl Contract for RadRunScoresContract {
    type Error = ViewError;
    type Storage = RadRunScores;
    type Message = ();
    type State = RadRunScores;
    type Query = ();

    async fn execute_operation(
        &mut self,
        operation: Operation,
    ) -> Result<Vec<Message>, Self::Error> {
        match operation {
            Operation::UpdateScore(score) => {
                self.state.scores.insert(&Owner::from([0; 32]), score)?;
                Ok(vec![])
            }
        }
    }
}

fn main() {}
