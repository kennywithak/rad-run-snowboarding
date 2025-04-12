// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use linera_sdk::base::{AbiError, AbiResult};
use linera_sdk::view::View;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RadRunScores {
    pub value: u64,
}

impl View for RadRunScores {
    type Error = AbiError;

    fn load() -> AbiResult<Self> {
        Ok(RadRunScores { value: 0 })
    }

    fn store(&self) -> AbiResult<()> {
        Ok(())
    }
} 