use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::serde::{ Serialize, Deserialize };
use near_sdk::AccountId;
use near_sdk::json_types::U128;
use crate::types::WrappedTimestamp;

/// Used on oracle to store global configuration
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OracleConfig {
    pub gov: AccountId,
    pub final_arbitrator: AccountId, // invoked to finalize when the `challenge_bond` for a data request is >= than `final_arbitrator_invoke_amount` / 100 % of the total supply
    pub stake_token: AccountId,
    pub payment_token: AccountId,
    pub validity_bond: U128,
    pub max_outcomes: u8,
    pub default_challenge_window_duration: WrappedTimestamp,
    pub min_initial_challenge_window_duration: WrappedTimestamp,
    pub final_arbitrator_invoke_amount: U128, // Amount of tokens that, when bonded in a single `ResolutionWindow`, should trigger the final arbitrator
    pub fee: FeeConfig,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct FeeConfig {
    // total market cap of FLUX/stake_token denominated in payment_token
    pub flux_market_cap: U128,
    // total value staked (TVS) of all request interfaces; denominated in payment_token
    pub total_value_staked: U128,
    // global percentage of TVS to pay out to resolutors; denominated in 1e5 so 1 = 0.001%, 100000 = 100%
    pub resolution_fee_percentage: u32,
}
