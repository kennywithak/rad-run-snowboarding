#![cfg_attr(target_arch = "wasm32", no_main)]

use async_trait::async_trait;
use linera_sdk::{
    base::{WithServiceAbi, Service},
    views::ViewStorageContext,
};
use rad_run_scores::{RadRunScoresAbi, RadRunScores};

pub struct RadRunScoresService {
    state: RadRunScores,
    runtime: ViewStorageContext,
}

impl WithServiceAbi for RadRunScoresService {
    type Abi = RadRunScoresAbi;
}

impl Service for RadRunScoresService {
    type Error = ();
    type Storage = RadRunScores;
    type Context = ViewStorageContext;

    async fn new(context: Self::Context) -> Result<Self, Self::Error> {
        let state = RadRunScores::load(&context);
        Ok(Self { state, runtime: context })
    }

    async fn handle_query(
        &self,
        _query: <Self::Abi as linera_sdk::base::ServiceAbi>::Query,
    ) -> Result<<Self::Abi as linera_sdk::base::ServiceAbi>::QueryResponse, Self::Error> {
        Ok(self.state.get_score())
    }
}

#[cfg(feature = "service")]
#[no_mangle]
pub fn service_main() {
    linera_sdk::service::service_main::<RadRunScoresService>();
}

#[no_mangle]
pub fn init() {
    linera_sdk::base::init::<RadRunScoresService>();
}

#[cfg(target_family = "wasm")]
#[no_mangle]
pub fn __wasm_call_ctors() {
    linera_sdk::service::init::<RadRunScoresService>();
} 