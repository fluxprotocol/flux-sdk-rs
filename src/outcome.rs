use near_sdk::json_types::U128;
use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };

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
