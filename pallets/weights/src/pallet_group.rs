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

//! Autogenerated weights for pallet_group
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
// -p=pallet_group
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

/// Weights for pallet_group using the Substrate node and recommended hardware.
pub struct WeightInfo;
impl pallet_group::WeightInfo for WeightInfo {
    // Storage: Instance2Group ActiveMembersLimit (r:1 w:1)
    fn set_active_members_limit() -> Weight {
        (30_672_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Instance2Group ActiveMembers (r:1 w:1)
    // Storage: Instance2Group ActiveMembersLimit (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Identity Claims (r:1 w:1)
    // Storage: Identity CurrentDid (r:1 w:0)
    fn add_member() -> Weight {
        (1_528_157_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Storage: Instance2Group InactiveMembers (r:1 w:1)
    // Storage: Identity CurrentDid (r:1 w:0)
    fn remove_member() -> Weight {
        (325_770_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Instance2Group ActiveMembers (r:1 w:1)
    // Storage: Identity Claims (r:1 w:1)
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Instance2Group InactiveMembers (r:1 w:1)
    fn disable_member() -> Weight {
        (281_008_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    // Storage: Instance2Group ActiveMembers (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Identity Claims (r:2 w:2)
    // Storage: Identity CurrentDid (r:1 w:0)
    fn swap_member() -> Weight {
        (1_554_960_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    // Storage: Instance2Group ActiveMembersLimit (r:1 w:0)
    // Storage: Instance2Group ActiveMembers (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Identity Claims (r:2 w:2)
    // Storage: Identity CurrentDid (r:1 w:0)
    fn reset_members(m: u32) -> Weight {
        (3_401_075_000 as Weight)
            // Standard Error: 5_538_000
            .saturating_add((1_322_473_000 as Weight).saturating_mul(m as Weight))
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().reads((1 as Weight).saturating_mul(m as Weight)))
            .saturating_add(DbWeight::get().writes(2 as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
    }
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity DidRecords (r:1 w:0)
    // Storage: Instance2Group ActiveMembers (r:1 w:1)
    // Storage: Identity Claims (r:1 w:1)
    fn abdicate_membership() -> Weight {
        (378_008_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
}
