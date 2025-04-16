#![cfg_attr(target_arch = "wasm32", no_main)]

use std::sync::{Arc, Mutex};
use async_graphql::{Request, Response, Schema, EmptyMutation, EmptySubscription, Object};
use linera_sdk::base::WithServiceAbi;
use linera_sdk::{service, Service, ServiceRuntime, ViewStateStorage};

use crate::state::RadRunScores;
use crate::query::Query;
use crate::error::ServiceError;
use crate::RadRunScoresAbi;

pub struct RadRunScoresService {
    state: Arc<RadRunScores>,
    runtime: Arc<Mutex<ServiceRuntime<Self>>>,
}

service!(RadRunScoresService);

impl WithServiceAbi for RadRunScoresService {
    type Abi = RadRunScoresAbi;
}

#[derive(Clone)]
struct QueryRoot {
    service: RadRunScoresService,
}

#[Object]
impl QueryRoot {
    async fn top_scores(&self, limit: usize) -> Vec<(String, u64)> {
        let mut all_scores = Vec::new();
        let entries = self.service.state.scores.indices().await.unwrap();

        for owner in entries {
            if let Some(score) = self.service.state.scores.get(&owner).await.unwrap_or(None) {
                all_scores.push((owner.to_string(), score));
            }
        }

        all_scores.sort_by(|a, b| b.1.cmp(&a.1));
        all_scores.into_iter().take(limit).collect()
    }
}

#[linera_sdk::async_trait]
impl Service for RadRunScoresService {
    type Error = ServiceError;
    type State = RadRunScores;
    type Storage = ViewStateStorage<Self>;

    async fn new(state: Self::State, runtime: ServiceRuntime<Self>) -> Result<Self, Self::Error> {
        Ok(Self {
            state: Arc::new(state),
            runtime: Arc::new(Mutex::new(runtime)),
        })
    }

    async fn handle_query(&self, request: Request) -> Result<Response, Self::Error> {
        let schema = Schema::build(
            QueryRoot {
                service: self.clone(),
            },
            EmptyMutation,
            EmptySubscription
::contentReference[oaicite:0]{index=0}
 
