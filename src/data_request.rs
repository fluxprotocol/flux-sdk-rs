use std::collections::HashMap;
use crate::{
    outcome::Outcome,
    requester::Requester,
    resolution_window::{ResolutionWindow, ResolutionWindowSummary},
    types::{Duration, WrappedBalance, WrappedTimestamp},
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::Vector,
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
    AccountId, Balance,
};

/// The arguments sent in `msg` on `ft_transfer_call()` from Requester to oracle while creating a new data request
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct NewDataRequestArgs {
    pub sources: Option<Vec<Source>>,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub outcomes: Option<Vec<String>>,
    pub challenge_period: WrappedTimestamp,
    pub data_type: DataRequestDataType,
    pub provider: Option<AccountId>,
}

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
pub struct Source {
    pub end_point: String,
    pub multiplier: Option<String>, // To normalize all values to same decimal cope
    pub source_path: String,
    pub http_instructions: Option<HTTPMethods>, // If `None` is a GET request
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub enum HTTPMethods {
    Get(HTTPAttributes),
    Head(HTTPAttributes),
    Post(HTTPAttributes),
    Put(HTTPAttributes),
    Delete(HTTPAttributes),
    Connect(HTTPAttributes),
    Options(HTTPAttributes),
    Trace(HTTPAttributes),
    Patch(HTTPAttributes),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct HTTPAttributes {
    pub http_headers: HashMap<String, String>,
    pub http_body: Option<String>,
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
    pub outcome: Outcome,
}

/// Used on the oracle in `claim()` to return token payouts
pub struct ClaimRes {
    pub payment_token_payout: u128,
    pub stake_token_payout: u128,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum DataRequest {
    Active(ActiveDataRequest),
    Finalized(FinalizedDataRequest),
}

/// Used in the oracle to store all information associated with a data request
#[derive(BorshSerialize, BorshDeserialize)]
pub struct ActiveDataRequest {
    pub id: u64,
    pub description: Option<String>,
    pub sources: Vec<Source>,
    pub outcomes: Option<Vec<String>>,
    pub requester: Requester, // Requester contract
    pub resolution_windows: Vector<ResolutionWindow>,
    pub global_config_id: u64, // Config id
    pub request_config: DataRequestConfig,
    pub initial_challenge_period: Duration, // challenge period for first resolution
    pub final_arbitrator_triggered: bool,
    pub tags: Vec<String>,
    pub data_type: DataRequestDataType,
    pub provider: Option<AccountId>, // For first party data_requests, expects to be immediately resolved with tags[0]
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FinalizedDataRequest {
    pub id: u64,
    pub finalized_outcome: Outcome,
    pub resolution_windows: Vector<ResolutionWindow>,
    pub global_config_id: u64, // Config id
    pub paid_fee: u128,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DataRequestConfig {
    pub default_challenge_window_duration: Duration,
    pub final_arbitrator_invoke_amount: Balance,
    pub final_arbitrator: AccountId,
    pub validity_bond: Balance,
    pub paid_fee: Balance,
    pub stake_multiplier: Option<u16>,
    pub min_resolution_bond: Balance,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum DataRequestSummary {
    Active(ActiveDataRequestSummary),
    Finalized(FinalizedDataRequestSummary),
}

/// Used on the oracle in `summarize_dr()` to return summary information about a data request
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct FinalizedDataRequestSummary {
    pub id: U64,
    pub finalized_outcome: Outcome,
    pub resolution_windows: Vec<ResolutionWindowSummary>,
    pub global_config_id: U64,
    pub paid_fee: U128,
}

/// Used on the oracle in `summarize_dr()` to return summary information about a data request
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ActiveDataRequestSummary {
    pub id: U64,
    pub description: Option<String>,
    pub sources: Vec<Source>,
    pub outcomes: Option<Vec<String>>,
    pub requester: Requester,
    pub request_config: DataRequestConfigSummary,
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
    pub min_resolution_bond: WrappedBalance,

}
