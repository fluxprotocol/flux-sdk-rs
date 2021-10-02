use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::{ AccountId, Balance };
use near_sdk::json_types::{ U64, U128 };
use near_sdk::collections::Vector;
use crate::types::{ Duration, WrappedBalance, WrappedTimestamp };
use crate::outcome::Outcome;
use crate::requester::Requester;
use crate::resolution_window::{ ResolutionWindow, ResolutionWindowSummary };

/// The arguments sent in `msg` on `ft_transfer_call()` from Requester to oracle while creating a new data request
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct NewDataRequestArgs {
    pub sources: Option<Vec<Source>>,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub outcomes: Option<Vec<String>>,
    pub challenge_period: WrappedTimestamp,
    pub data_type: DataRequestDataType,
}

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
pub struct Source {
    pub end_point: String,
    pub source_path: String
}

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum DataRequestDataType {
    Number(U128),
    String,
}

/// The arguments sent in `msg` on `ft_transfer_call()` from Requester to oracle while staking on a data request
#[derive(Serialize, Deserialize)]
pub struct StakeDataRequestArgs {
    pub id: U64,
    pub outcome: Outcome
}

/// Used on the oracle in `claim()` to return token payouts
pub struct ClaimRes {
    pub payment_token_payout: u128,
    pub stake_token_payout: u128
}

/// Used in the oracle to store all information associated with a data request
#[derive(BorshSerialize, BorshDeserialize)]
pub struct DataRequest {
    pub id: u64,
    pub description: Option<String>,
    pub sources: Vec<Source>,
    pub outcomes: Option<Vec<String>>,
    pub requester: Requester, // Requester contract
    pub finalized_outcome: Option<Outcome>,
    pub resolution_windows: Vector<ResolutionWindow>,
    pub global_config_id: u64, // Config id
    pub request_config: DataRequestConfig,
    pub initial_challenge_period: Duration, // challenge period for first resolution
    pub final_arbitrator_triggered: bool,
    pub tags: Vec<String>,
    pub data_type: DataRequestDataType,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DataRequestConfig {
    default_challenge_window_duration: Duration,
    final_arbitrator_invoke_amount: Balance,
    final_arbitrator: AccountId,
    validity_bond: Balance,
    pub paid_fee: Balance,
    pub stake_multiplier: Option<u16>,
}

/// Used on the oracle in `summarize_dr()` to return summary information about a data request
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DataRequestSummary {
    pub id: u64,
    pub description: Option<String>,
    pub sources: Vec<Source>,
    pub outcomes: Option<Vec<String>>,
    pub requester: Requester,
    pub request_config: DataRequestConfigSummary,
    pub finalized_outcome: Option<Outcome>,
    pub resolution_windows: Vec<ResolutionWindowSummary>,
    pub global_config_id: U64,
    pub initial_challenge_period: U64,
    pub final_arbitrator_triggered: bool,
    pub tags: Vec<String>,
    pub data_type: DataRequestDataType,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DataRequestConfigSummary {
    pub validity_bond: WrappedBalance,
    pub paid_fee: WrappedBalance,
    pub stake_multiplier: Option<u16>,
}