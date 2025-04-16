use linera_sdk::abi::{ContractAbi, ServiceAbi, Abi};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    SetScore { score: u64 },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Query {
    GetTopScores { count: u32 },
}

pub struct RadRunScoresAbi;

impl ContractAbi for RadRunScoresAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for RadRunScoresAbi {
    type Query = Query;
    type QueryResponse = Vec<(String, u64)>;
}

impl Abi for RadRunScoresAbi {}
