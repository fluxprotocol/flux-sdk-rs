use near_sdk::{
    serde::{ Deserialize, Serialize },
    borsh::{ self, BorshDeserialize, BorshSerialize },
    AccountId
};
use crate::{
    outcome::Outcome,
    types::WrappedBalance,
    data_request::NewDataRequestArgs
};

/// Used on sample Requester contract to keep track of created data requests
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize)]
pub struct DataRequestDetails {
    pub amount: WrappedBalance,
    pub payload: NewDataRequestArgs,
    pub tags: Vec<String>,
    pub status: RequestStatus,
    pub creator: AccountId,
    pub has_withdrawn_validity_bond: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum RequestStatus {
    Pending,
    Finalized(Outcome)
}

/// An entry in the Requester Registry
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct Requester {
    pub contract_name: String,
    pub account_id: AccountId,
    pub stake_multiplier: Option<u16>, 
    pub code_base_url: Option<String>
}
