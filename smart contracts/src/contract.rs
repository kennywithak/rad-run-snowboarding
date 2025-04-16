#![cfg_attr(target_arch = "wasm32", no_main)]

use linera_sdk::base::WithContractAbi;
use linera_sdk::{contract, Contract, ContractRuntime};

use crate::abi::{Operation, Parameters};
use crate::state::RadRunScores;
use crate::RadRunScoresAbi;

pub struct RadRunScoresContract {
    state: RadRunScores,
    runtime: ContractRuntime<Self>,
}

contract!(RadRunScoresContract);

impl WithContractAbi for RadRunScoresContract {
    type Abi = RadRunScoresAbi;
}

#[linera_sdk::async_trait]
impl Contract for RadRunScoresContract {
    type Message = ();
    type InstantiationArgument = ();
    type Parameters = Parameters;

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = RadRunScores::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Self { state, runtime }
    }

    async fn instantiate(&mut self, _arg: Self::InstantiationArgument) {}

    async fn execute_operation(&mut self, operation: Operation) {
        match operation {
            Operation::SetScore { player: _, score } => {
                let owner = self.runtime.authenticated_signer().expect("Not signed in");
                let current = self.state.scores.get(&owner).await.unwrap_or(Some(0)).unwrap();
                if score > current {
                    self.state.scores.insert(&owner, score).await.expect("Failed to insert score");
                }
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to store state");
    }
}
