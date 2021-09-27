use near_sdk::AccountId;
use near_sdk::json_types::{ U64, U128 };
use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };

pub type Duration = u64;
pub type Timestamp = u64;
pub type WrappedTimestamp = U64;
pub type WrappedBalance = U128;

pub struct ClaimRes {
    pub payment_token_payout: u128,
    pub stake_token_payout: u128
}

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum Outcome {
    Answer(AnswerType),
    Invalid
}

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum AnswerType {
    Number(AnswerNumberType),
    String(String)
}

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct AnswerNumberType {
    pub value: U128,
    pub multiplier: U128,
    pub negative: bool,
}


#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
pub struct Source {
    pub end_point: String,
    pub source_path: String
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum RequestStatus {
    Pending,
    Finalized(Outcome)
}

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum DataRequestDataType {
    Number(U128),
    String,
}

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize)]
pub struct DataRequest {
    pub amount: WrappedBalance,
    pub payload: NewDataRequestArgs,
    pub tags: Vec<String>,
    pub status: RequestStatus
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct NewDataRequestArgs {
    pub sources: Option<Vec<Source>>,
    pub tags: Option<Vec<String>>,
    pub description: Option<String>,
    pub outcomes: Option<Vec<String>>,
    pub challenge_period: WrappedTimestamp,
    pub data_type: DataRequestDataType,
    pub creator: AccountId,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Nonce(u64);
impl Nonce {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn get_and_incr(&mut self) -> u64 {
        let val = self.0;
        self.0 += 1;
        val
    }
}
