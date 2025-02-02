// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_committee
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-22, STEPS: `100`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 512

// Executed Command:
// ./target/release/polymesh
// benchmark
// pallet
// -s
// 100
// -r
// 5
// -p=pallet_committee
// -e=*
// --heap-pages
// 4096
// --db-cache
// 512
// --execution
// wasm
// --wasm-execution
// compiled
// --output
// ./pallets/weights/src/
// --template
// ./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use polymesh_runtime_common::{RocksDbWeight as DbWeight, Weight};

/// Weights for pallet_committee using the Substrate node and recommended hardware.
pub struct WeightInfo;
impl pallet_committee::WeightInfo for WeightInfo {
    // Storage: Instance1Committee VoteThreshold (r:0 w:1)
    fn set_vote_threshold() -> Weight {
        (20_972_000 as Weight).saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Instance1Committee Members (r:1 w:0)
    // Storage: Instance1Committee ReleaseCoordinator (r:0 w:1)
    fn set_release_coordinator() -> Weight {
        (166_590_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Instance1Committee ExpiresAfter (r:0 w:1)
    fn set_expires_after() -> Weight {
        (24_033_000 as Weight).saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Instance1Committee Voting (r:1 w:1)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Instance1Committee Members (r:1 w:0)
    // Storage: Instance1Committee ProposalCount (r:1 w:1)
    // Storage: Instance1Committee ProposalOf (r:1 w:1)
    // Storage: Instance1Committee Proposals (r:1 w:1)
    // Storage: Instance1Committee ExpiresAfter (r:1 w:0)
    fn vote_or_propose_new_proposal() -> Weight {
        (396_265_000 as Weight)
            .saturating_add(DbWeight::get().reads(7 as Weight))
            .saturating_add(DbWeight::get().writes(4 as Weight))
    }
    // Storage: Instance1Committee Voting (r:1 w:1)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Instance1Committee Members (r:1 w:0)
    // Storage: Instance1Committee VoteThreshold (r:1 w:0)
    fn vote_or_propose_existing_proposal() -> Weight {
        (309_183_000 as Weight)
            .saturating_add(DbWeight::get().reads(4 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Instance1Committee Members (r:1 w:0)
    // Storage: Instance1Committee Voting (r:1 w:1)
    // Storage: Instance1Committee VoteThreshold (r:1 w:0)
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Instance1Committee ProposalOf (r:1 w:1)
    // Storage: Instance1Committee Proposals (r:1 w:1)
    fn vote_aye() -> Weight {
        (550_119_000 as Weight)
            .saturating_add(DbWeight::get().reads(7 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Instance1Committee Members (r:1 w:0)
    // Storage: Instance1Committee Voting (r:1 w:1)
    // Storage: Instance1Committee VoteThreshold (r:1 w:0)
    fn vote_nay() -> Weight {
        (332_290_000 as Weight)
            .saturating_add(DbWeight::get().reads(4 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
}
