use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::{ Balance, AccountId };
use near_sdk::collections::{ LookupMap };
use crate::outcome::Outcome;
use crate::types::{ Timestamp, WrappedBalance, WrappedTimestamp };

/// Used in the oracle to store all information associated with a resolution window within a data request
#[derive(BorshSerialize, BorshDeserialize)]
pub struct ResolutionWindow {
    pub dr_id: u64,
    pub round: u16,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub bond_size: Balance,
    pub outcome_to_stake: LookupMap<Outcome, Balance>,
    pub user_to_outcome_to_stake: LookupMap<AccountId, LookupMap<Outcome, Balance>>,
    pub bonded_outcome: Option<Outcome>,
}

pub struct CorrectStake {
    pub bonded_stake: Balance,
    pub user_stake: Balance,
}

pub enum WindowStakeResult {
    Incorrect(Balance),    // Round bonded outcome was correct
    Correct(CorrectStake), // Round bonded outcome was incorrect
    NoResult               // Last / non-bonded window
}

/// Used on the oracle in `summarize_dr()` to return summary information about a data request
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ResolutionWindowSummary {
    pub round: u16,
    pub start_time: WrappedTimestamp, 
    pub end_time: WrappedTimestamp,
    pub bond_size: WrappedBalance,
    pub bonded_outcome: Option<Outcome>
}
