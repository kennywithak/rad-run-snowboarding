// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use linera_sdk::{
    base::{ContractAbi, ServiceAbi},
    views::{ViewStorageContext, RegisterView, RootView},
};

#[derive(RootView)]
#[view(context = "ViewStorageContext")]
pub struct RadRunScores {
    pub score: RegisterView<u64>,
}

impl RadRunScores {
    pub fn set_score(&mut self, value: u64) {
        self.score.set(value);
    }

    pub fn get_score(&self) -> u64 {
        *self.score.get()
    }
}

#[derive(Debug)]
pub struct RadRunScoresAbi;

impl ContractAbi for RadRunScoresAbi {
    type Operation = u64;
    type Response = u64;
}

impl ServiceAbi for RadRunScoresAbi {
    type Query = ();
    type QueryResponse = u64;
}

pub type RadRunScoresContract = RadRunScores;
pub type RadRunScoresService = RadRunScores;
