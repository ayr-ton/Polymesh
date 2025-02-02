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

//! Autogenerated weights for pallet_pips
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-25, STEPS: `100`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 512

// Executed Command:
// ./target/release/polymesh
// benchmark
// pallet
// -s
// 100
// -r
// 5
// -p=pallet_pips
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

/// Weights for pallet_pips using the Substrate node and recommended hardware.
pub struct WeightInfo;
impl pallet_pips::WeightInfo for WeightInfo {
    // Storage: Pips PruneHistoricalPips (r:1 w:1)
    fn set_prune_historical_pips() -> Weight {
        (27_104_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Pips MinimumProposalDeposit (r:1 w:1)
    fn set_min_proposal_deposit() -> Weight {
        (26_912_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Pips DefaultEnactmentPeriod (r:1 w:1)
    fn set_default_enactment_period() -> Weight {
        (25_513_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Pips PendingPipExpiry (r:1 w:1)
    fn set_pending_pip_expiry() -> Weight {
        (26_616_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Pips MaxPipSkipCount (r:1 w:1)
    fn set_max_pip_skip_count() -> Weight {
        (25_032_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Pips ActivePipLimit (r:1 w:1)
    fn set_active_pip_limit() -> Weight {
        (25_431_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Pips PipIdSequence (r:1 w:1)
    // Storage: Pips ActivePipLimit (r:1 w:0)
    // Storage: Pips ActivePipCount (r:1 w:1)
    // Storage: Pips MinimumProposalDeposit (r:1 w:0)
    // Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Storage: Pips PendingPipExpiry (r:1 w:0)
    // Storage: Pips ProposalResult (r:1 w:1)
    // Storage: Pips ProposalVotes (r:1 w:1)
    // Storage: Pips LiveQueue (r:1 w:1)
    // Storage: Pips ProposalMetadata (r:0 w:1)
    // Storage: Pips Deposits (r:0 w:1)
    // Storage: Pips Proposals (r:0 w:1)
    fn propose_from_community() -> Weight {
        (203_106_000 as Weight)
            .saturating_add(DbWeight::get().reads(14 as Weight))
            .saturating_add(DbWeight::get().writes(11 as Weight))
    }
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Pips PipIdSequence (r:1 w:1)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Storage: Pips PendingPipExpiry (r:1 w:0)
    // Storage: Pips ActivePipCount (r:1 w:1)
    // Storage: Pips CommitteePips (r:1 w:1)
    // Storage: Pips ProposalMetadata (r:0 w:1)
    // Storage: Pips Proposals (r:0 w:1)
    fn propose_from_committee() -> Weight {
        (109_791_000 as Weight)
            .saturating_add(DbWeight::get().reads(7 as Weight))
            .saturating_add(DbWeight::get().writes(5 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Pips Proposals (r:1 w:0)
    // Storage: Pips ProposalResult (r:1 w:1)
    // Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
    // Storage: Pips Deposits (r:1 w:1)
    // Storage: Pips ProposalVotes (r:1 w:1)
    // Storage: Pips LiveQueue (r:1 w:1)
    fn vote() -> Weight {
        (238_241_000 as Weight)
            .saturating_add(DbWeight::get().reads(7 as Weight))
            .saturating_add(DbWeight::get().writes(5 as Weight))
    }
    // Storage: Pips Proposals (r:1 w:1)
    // Storage: Pips DefaultEnactmentPeriod (r:1 w:0)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Pips PipToSchedule (r:0 w:1)
    fn approve_committee_proposal() -> Weight {
        (114_882_000 as Weight)
            .saturating_add(DbWeight::get().reads(4 as Weight))
            .saturating_add(DbWeight::get().writes(4 as Weight))
    }
    // Storage: Pips Proposals (r:1 w:1)
    // Storage: Pips LiveQueue (r:1 w:1)
    // Storage: Pips SnapshotMeta (r:1 w:0)
    // Storage: Pips ActivePipCount (r:1 w:1)
    // Storage: Pips PruneHistoricalPips (r:1 w:0)
    // Storage: Pips Deposits (r:2 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Pips ProposalVotes (r:0 w:1)
    // Storage: Pips ProposalMetadata (r:0 w:1)
    // Storage: Pips PipSkipCount (r:0 w:1)
    // Storage: Pips ProposalResult (r:0 w:1)
    fn reject_proposal() -> Weight {
        (196_681_000 as Weight)
            .saturating_add(DbWeight::get().reads(9 as Weight))
            .saturating_add(DbWeight::get().writes(10 as Weight))
    }
    // Storage: Pips Proposals (r:1 w:1)
    // Storage: Pips Deposits (r:1 w:0)
    // Storage: Pips ProposalVotes (r:0 w:1)
    // Storage: Pips ProposalMetadata (r:0 w:1)
    // Storage: Pips PipSkipCount (r:0 w:1)
    // Storage: Pips ProposalResult (r:0 w:1)
    fn prune_proposal() -> Weight {
        (99_211_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(5 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Instance1Committee ReleaseCoordinator (r:1 w:0)
    // Storage: Pips Proposals (r:1 w:0)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Pips PipToSchedule (r:0 w:1)
    fn reschedule_execution() -> Weight {
        (111_182_000 as Weight)
            .saturating_add(DbWeight::get().reads(6 as Weight))
            .saturating_add(DbWeight::get().writes(4 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Instance1Committee Members (r:1 w:0)
    // Storage: Pips SnapshotMeta (r:1 w:1)
    // Storage: Pips SnapshotQueue (r:0 w:1)
    fn clear_snapshot() -> Weight {
        (55_742_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Instance1Committee Members (r:1 w:0)
    // Storage: Pips SnapshotIdSequence (r:1 w:1)
    // Storage: Pips LiveQueue (r:1 w:0)
    // Storage: Pips SnapshotQueue (r:0 w:1)
    // Storage: Pips SnapshotMeta (r:0 w:1)
    fn snapshot() -> Weight {
        (1_049_777_000 as Weight)
            .saturating_add(DbWeight::get().reads(4 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    // Storage: Pips MaxPipSkipCount (r:1 w:0)
    // Storage: Pips SnapshotQueue (r:1 w:1)
    // Storage: Pips PipSkipCount (r:33 w:33)
    // Storage: Pips LiveQueue (r:1 w:1)
    // Storage: Pips Proposals (r:33 w:33)
    // Storage: Pips ActivePipCount (r:1 w:1)
    // Storage: Pips PruneHistoricalPips (r:1 w:0)
    // Storage: Pips Deposits (r:13233 w:13200)
    // Storage: Balances Locks (r:400 w:400)
    // Storage: System Account (r:400 w:400)
    // Storage: Pips SnapshotMeta (r:1 w:0)
    // Storage: Pips DefaultEnactmentPeriod (r:1 w:0)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Pips PipToSchedule (r:0 w:1)
    fn enact_snapshot_results(a: u32, r: u32, s: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 120_040_000
            .saturating_add((431_560_000 as Weight).saturating_mul(a as Weight))
            // Standard Error: 120_040_000
            .saturating_add((17_425_641_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(DbWeight::get().reads(689 as Weight))
            .saturating_add(DbWeight::get().reads((3 as Weight).saturating_mul(a as Weight)))
            .saturating_add(DbWeight::get().reads((405 as Weight).saturating_mul(r as Weight)))
            .saturating_add(DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
            .saturating_add(DbWeight::get().writes(685 as Weight))
            .saturating_add(DbWeight::get().writes((4 as Weight).saturating_mul(a as Weight)))
            .saturating_add(DbWeight::get().writes((404 as Weight).saturating_mul(r as Weight)))
            .saturating_add(DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Pips Proposals (r:1 w:1)
    // Storage: Pips ActivePipCount (r:1 w:1)
    // Storage: Pips PruneHistoricalPips (r:1 w:0)
    // Storage: Pips Deposits (r:401 w:400)
    // Storage: Balances Locks (r:400 w:400)
    // Storage: System Account (r:400 w:400)
    // Storage: Pips ProposalVotes (r:0 w:400)
    // Storage: Pips ProposalMetadata (r:0 w:1)
    // Storage: Pips PipSkipCount (r:0 w:1)
    // Storage: Pips PipToSchedule (r:0 w:1)
    // Storage: Pips ProposalResult (r:0 w:1)
    fn execute_scheduled_pip() -> Weight {
        (24_436_343_000 as Weight)
            .saturating_add(DbWeight::get().reads(1204 as Weight))
            .saturating_add(DbWeight::get().writes(1606 as Weight))
    }
    // Storage: Pips Proposals (r:1 w:1)
    // Storage: Pips LiveQueue (r:1 w:1)
    // Storage: Pips SnapshotMeta (r:1 w:0)
    // Storage: Pips SnapshotQueue (r:1 w:1)
    // Storage: Pips ActivePipCount (r:1 w:1)
    // Storage: Pips PruneHistoricalPips (r:1 w:0)
    // Storage: Pips Deposits (r:401 w:400)
    // Storage: Balances Locks (r:400 w:400)
    // Storage: System Account (r:400 w:400)
    // Storage: Pips ProposalVotes (r:0 w:400)
    // Storage: Pips ProposalMetadata (r:0 w:1)
    // Storage: Pips PipSkipCount (r:0 w:1)
    // Storage: Pips ProposalResult (r:0 w:1)
    fn expire_scheduled_pip() -> Weight {
        (26_587_065_000 as Weight)
            .saturating_add(DbWeight::get().reads(1207 as Weight))
            .saturating_add(DbWeight::get().writes(1607 as Weight))
    }
}
