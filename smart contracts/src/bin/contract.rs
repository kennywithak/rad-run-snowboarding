#![cfg_attr(target_arch = "wasm32", no_main)]

use linera_sdk::{base::WithContractAbi, contract::Contract, views::ViewStorageContext};
use rad_run_scores::{RadRunScores, RadRunScoresAbi};

pub struct RadRunScoresContract {
    state: RadRunScores,
    runtime: ViewStorageContext,
}

impl WithContractAbi for RadRunScoresContract {
    type Abi = RadRunScoresAbi;
}

impl Contract for RadRunScoresContract {
    type InstantiationArgument = u64;
    type Message = ();

    async fn instantiate(
        mut runtime: ViewStorageContext,
        argument: Self::InstantiationArgument,
    ) -> Result<Self, ()> {
        let mut state = RadRunScores::new(&runtime);
        state.set_score(argument);
        Ok(Self { state, runtime })
    }

    async fn execute_operation(
        &mut self,
        operation: <Self::Abi as linera_sdk::base::ContractAbi>::Operation,
    ) -> Result<<Self::Abi as linera_sdk::base::ContractAbi>::Response, ()> {
        self.state.set_score(operation);
        Ok(())
    }

    async fn execute_message(
        &mut self,
        _message: Self::Message,
    ) -> Result<(), ()> {
        Ok(())
    }

    async fn load(runtime: ViewStorageContext) -> Self {
        let state = RadRunScores::new(&runtime);
        Self { state, runtime }
    }

    async fn store(self) {
        self.state.store().await;
    }
}

#[cfg(feature = "contract")]
#[no_mangle]
pub fn init() {
    linera_sdk::contract::init::<RadRunScoresContract>();
} 