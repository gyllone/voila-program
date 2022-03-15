use borsh::{BorshSerialize, BorshDeserialize};
use serde::{Serialize, Deserialize};
use solana_program::{
    pubkey::Pubkey,
    program_pack::IsInitialized,
    clock::{UnixTimestamp, Clock},
};

use crate::Packer;

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct KeyInfo {
    pub is_initialized: bool,
    pub admin: Pubkey,
    pub receipt: Pubkey,
    pub price: u64,
}

impl KeyInfo {
    pub fn new(
        admin: Pubkey,
        receipt: Pubkey,
        price: u64,
    ) -> Self {
        Self {
            is_initialized: true,
            admin,
            receipt,
            price,
        }
    }
}

impl IsInitialized for KeyInfo {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Packer for KeyInfo {
    const LEN: usize = 1 + 32 + 32 + 8;
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct UserKeyRecord {
    pub is_initialized: bool,
    pub key_info: Pubkey,
    pub timestamp: UnixTimestamp,
    pub price: u64,
}

impl UserKeyRecord {
    pub fn new(
        key_info: Pubkey,
        clock: &Clock,
        price: u64,
    ) -> Self {
        Self {
            is_initialized: true,
            key_info,
            timestamp: clock.unix_timestamp,
            price,
        }
    }
}

impl IsInitialized for UserKeyRecord {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Packer for UserKeyRecord {
    const LEN: usize = 1 + 8 + 8;
}
