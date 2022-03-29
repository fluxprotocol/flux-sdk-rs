use near_sdk::Gas;

pub const PERCENTAGE_DIVISOR: u16 = 10_000;
pub const MAX_SOURCES: u8 = 8;
pub const MAX_TAGS: u8 = 8;
pub const MIN_OUTCOMES: u8 = 2;
pub const MIN_PERIOD_MULTIPLIER: u64 = 3;

pub const MAX_RESOLUTION_FEE_PERCENTAGE: u32 = 5000; // 5% in 1e5

// Gas definitions
pub const GAS_BASE_TRANSFER: Gas = Gas(5_000_000_000_000);
pub const FINALIZATION_GAS: u64 = 250_000_000_000_000;
pub const MAX_GAS: u64 = 300_000_000_000_000;
pub const DR_NEW_GAS: Gas = Gas(200_000_000_000_000);
pub const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(5_000_000_000_000);
pub const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(30_000_000_000_000); // includes GAS_FOR_RESOLVE_TRANSFER
pub const GAS_BASE_SET_OUTCOME: Gas = Gas(250_000_000_000_000);
