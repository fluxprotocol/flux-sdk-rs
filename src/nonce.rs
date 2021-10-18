use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

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
