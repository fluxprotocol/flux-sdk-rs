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
pub struct NewDataRequestBase {
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub data_type: DataRequestDataType,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct NewFetchRequestArgs {
    pub sources: Option<Vec<Source>>,
    pub dr_args: NewDataRequestBase,
    pub challenge_period: WrappedTimestamp,
    pub data_type: DataRequestDataType,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct NewCrowdSourcedRequestArgs {
    pub outcomes: Option<Vec<String>>,
    pub dr_args: NewDataRequestBase,
    pub challenge_period: WrappedTimestamp,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct NewProviderRequestArgs {
    pub tags: Vec<String>,
    pub provider: AccountId,
    pub dr_args: NewDataRequestBase,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub enum NewDataRequestArgs {
    NewFetchRequestArgs(NewFetchRequestArgs),
    NewCrowdSourcedRequestArgs(NewCrowdSourcedRequestArgs),
    NewProviderRequestArgs(NewProviderRequestArgs),
}

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
pub struct Source {
    pub end_point: String,
    pub source_path: String,
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

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Debug, PartialEq)]
enum Active {
    CrowdSourced,
    Provider,
    Fetch,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum ActiveDataRequest {
    FetchDataRequest(FetchDataRequest),
    CrowdSourcedDataRequest(CrowdSourcedDataRequest),
    ProviderDataRequest(ProviderDataRequest),
}

/// Used in the oracle to store all information associated with a data request
#[derive(BorshSerialize, BorshDeserialize)]
pub struct DisputableDataRequestBase {
    pub id: u64,
    pub requester: Requester, // Requester contract
    pub resolution_windows: Vector<ResolutionWindow>,
    pub global_config_id: u64, // Config id
    pub request_config: DataRequestConfig,
    pub initial_challenge_period: Duration, // challenge period for first resolution
    pub final_arbitrator_triggered: bool,
    pub tags: Vec<String>,
    pub data_type: DataRequestDataType,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ProviderDataRequest {
    pub id: u64,
    pub tags: Vec<String>,
    pub provider: AccountId,
    pub requester: Requester,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FetchDataRequest {
    pub base_data: DisputableDataRequestBase,
    pub sources: Vec<Source>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CrowdSourcedDataRequest {
    pub base_data: DisputableDataRequestBase,
    pub sources: Vec<Source>,
    pub description: Option<String>,
    pub outcomes: Option<Vec<String>>,
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
}
