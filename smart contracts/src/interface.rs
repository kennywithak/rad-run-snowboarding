// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{Context, Object, Schema, SimpleObject};
use linera_sdk::{
    base::{ApplicationId, SessionId},
    views::{View, ViewError},
    Service,
};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use linera_sdk::{
    base::{WithInterfaceAbi, InterfaceAbi},
    interface::Interface,
};
use crate::{RadRunScores, RadRunScoresAbi};

#[derive(Debug, Deserialize, Serialize)]
pub enum Query {
    GetScore,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Score {
    pub value: u64,
}

pub struct RadRunScoresInterface {
    state: RadRunScores,
}

impl WithInterfaceAbi for RadRunScoresInterface {
    type Abi = RadRunScoresAbi;
}

#[async_trait]
impl Interface for RadRunScoresInterface {
    type Query = Query;
    type Error = ViewError;

    async fn query(&self, query: Self::Query) -> Result<Score, Self::Error> {
        match query {
            Query::GetScore => {
                let score = self.state.get_score().await?;
                Ok(Score { value: score })
            }
        }
    }
}
