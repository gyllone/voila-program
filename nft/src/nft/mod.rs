#[cfg(feature = "metaplex")]
pub mod metaplex;

use borsh::{BorshSerialize, BorshDeserialize};
use serde::{Serialize, Deserialize};
use solana_program::{
    pubkey::Pubkey,
    program_pack::IsInitialized,
};

use crate::{Packer, pda::get_common_nft_authority_pda};

pub trait Meta<Data: Sized> {
    fn metadata(&self, mint: &Pubkey) -> Data;
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct CommonNFTInfo {
    pub is_initialized: bool,
    pub admin: Pubkey,
    pub receipt: Pubkey,
    pub pda_authority: Pubkey,
    pub pda_seed: [u8; 1],
    pub price: u64,
    pub max_amount: u16,
    pub current_amount: u16,
    pub name: String,
    pub uri: String,
}

impl IsInitialized for CommonNFTInfo {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Packer for CommonNFTInfo {
    const LEN: usize = 512;
}

impl CommonNFTInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        admin: Pubkey,
        receipt: Pubkey,
        nft_pubkey: &Pubkey,
        program_id: &Pubkey,
        price: u64,
        max_amount: u16,
        name: String,
        uri: String,
    ) -> Self {
        let (pda_authority, _, pda_seed)
            = get_common_nft_authority_pda(nft_pubkey, program_id);

        Self {
            is_initialized: true,
            admin,
            receipt,
            pda_authority,
            pda_seed,
            price,
            max_amount,
            current_amount: 0,
            name,
            uri,
        }
    }

    #[inline]
    pub fn authority_signer_seeds<'a>(&'a self, nft_pubkey: &'a Pubkey) -> [&'a [u8]; 2] {
        [nft_pubkey.as_ref(), &self.pda_seed]
    }
}
