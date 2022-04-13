use borsh::{BorshSerialize, BorshDeserialize};
use serde::{Serialize, Deserialize};
use solana_program::{msg, pubkey::Pubkey, clock::UnixTimestamp, program_error::ProgramError, program_pack::IsInitialized, entrypoint::ProgramResult};

use crate::{pda::get_nft_auction_authority_pda, error::VoilaError, Packer};

#[derive(Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct NFTAuction {
    pub is_initialized: bool,
    pub admin: Pubkey,
    pub pda_authority: Pubkey,
    pub pda_seed: [u8; 1],
    pub start_time: UnixTimestamp,
    pub end_time: UnixTimestamp,
    pub base_price: u64,
    pub min_raise_price: u64,
    pub last_price: u64,
    pub bidder: Option<Pubkey>,
    pub claimed: bool,
    pub name: String,
    pub uri: String,
}

impl NFTAuction {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        admin: Pubkey,
        nft_auction: &Pubkey,
        program_id: &Pubkey,
        start_time: UnixTimestamp,
        end_time: UnixTimestamp,
        base_price: u64,
        min_raise_price: u64,
        name: String,
        uri: String,
    ) -> Self {
        let (pda_authority, _, pda_seed)
            = get_nft_auction_authority_pda(nft_auction, program_id);

        Self {
            is_initialized: true,
            admin,
            pda_authority,
            pda_seed,
            start_time,
            end_time,
            base_price,
            min_raise_price,
            last_price: base_price,
            bidder: None,
            claimed: false,
            name,
            uri,
        }
    }

    #[inline]
    pub fn authority_signer_seeds<'a>(&'a self, nft_auction: &'a Pubkey) -> [&'a [u8]; 2] {
        [nft_auction.as_ref(), &self.pda_seed]
    }

    pub fn bid(
        &mut self,
        raise_price: u64,
        timestamp: UnixTimestamp,
        bidder: Pubkey,
    ) -> Result<(Option<Pubkey>, u64), ProgramError> {
        if raise_price < self.min_raise_price {
            return Err(VoilaError::InvalidBidPrice.into());
        }

        if timestamp < self.start_time {
            msg!("Auction is not started yet");
            return Err(VoilaError::InvalidBidTime.into());
        } else if timestamp > self.end_time {
            msg!("Auction is end of bidding");
            return Err(VoilaError::InvalidBidTime.into());
        }

        let refund = self.last_price;
        self.last_price = self.last_price
            .checked_add(raise_price)
            .ok_or(VoilaError::MathOverflow)?;

        Ok((self.bidder.replace(bidder), refund))
    }

    pub fn claim(&mut self, timestamp: UnixTimestamp, owner: &Pubkey) -> ProgramResult {
        if timestamp < self.end_time {
            msg!("Auction is not end of bidding yet");
            return Err(VoilaError::InvalidBidTime.into());
        }

        if let Some(bidder) = &self.bidder {
            if bidder == owner {
                self.claimed = true;

                Ok(())
            } else {
                msg!("Only the latest bidder can claim the NFT");
                Err(VoilaError::NFTCannotClaim.into())
            }
        } else {
            msg!("No bidder exists");
            Err(VoilaError::NFTCannotClaim.into())
        }
    }
}

impl IsInitialized for NFTAuction {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Packer for NFTAuction {
    const LEN: usize = 256;
}