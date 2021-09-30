use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::AccountId;
use crate::outcome::Outcome;
use crate::types::WrappedBalance;
use crate::data_request::NewDataRequestArgs;

/// Used on sample Requester contract to keep track of created data requests
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize)]
pub struct DataRequestDetails {
    pub amount: WrappedBalance,
    pub payload: NewDataRequestArgs,
    pub tags: Vec<String>,
    pub status: RequestStatus
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
    pub account_id: AccountId, // Change to account_id
    pub stake_multiplier: Option<u16>, 
    pub code_base_url: Option<String>
}
