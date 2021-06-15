//! State transition types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

use crate::PROGRAM_VERSION;

/// Accounts are created with data zeroed out, so uninitialized state instances
/// will have the version set to 0.
pub const UNINITIALIZED_VERSION: u8 = 0;

/// The the root entity within the program
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct RewardManager {
    pub version: u8,
    /// Token account for rewards to be sent via this program
    pub token_account: Pubkey,
    /// Account authorized managing this Reward Manager (adding/removing signers, updating params etc.)
    pub manager: Pubkey,
    /// Number of signer votes required for sending rewards
    pub min_votes: u8,
}

impl RewardManager {
    /// The struct size on bytes
    pub const LEN: usize = 66;

    /// Creates new `RewardManager`
    pub fn new(token_account: Pubkey, manager: Pubkey, min_votes: u8) -> Self {
        Self {
            version: PROGRAM_VERSION,
            token_account,
            manager,
            min_votes,
        }
    }
}

impl IsInitialized for RewardManager {
    fn is_initialized(&self) -> bool {
        self.version != UNINITIALIZED_VERSION
    }
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct SenderAccount {
    pub version: u8,
    pub reward_manager: Pubkey,
    pub eth_address: [u8; 20],
}

impl SenderAccount {
    /// The struct size on bytes
    pub const LEN: usize = 53;

    /// Creates new `SenderAccount`
    pub fn new(reward_manager: Pubkey, eth_address: [u8; 20]) -> Self {
        Self {
            version: PROGRAM_VERSION,
            reward_manager,
            eth_address,
        }
    }
}

impl IsInitialized for SenderAccount {
    fn is_initialized(&self) -> bool {
        self.version != UNINITIALIZED_VERSION
    }
}
