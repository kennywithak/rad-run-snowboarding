use linera_views::views::ViewError;
use linera_sdk::views::ViewStorageContext;
use crate::state::RadRunScores;
use linera_sdk::base::Owner;

pub type RadRunScoresInterface = RadRunScores;

pub struct RadRunInterface {
    pub state: RadRunScoresInterface,
}

impl RadRunInterface {
    pub fn new(context: ViewStorageContext) -> Result<Self, ViewError> {
        Ok(Self {
            state: RadRunScores::new(context)?,
        })
    }
}
