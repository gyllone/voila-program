use borsh::{BorshSerialize, BorshDeserialize};
use serde::{Serialize, Deserialize};
use solana_program::{msg, pubkey::Pubkey, clock::UnixTimestamp, program_error::ProgramError, program_pack::IsInitialized, entrypoint::ProgramResult};

use crate::{pda::get_nft_auction_authority_pda, error::VoilaError, Packer};

const PREVIOUS_BIDDERS_LEN: usize = 6;

#[derive(Clone, Copy, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct BidInfo {
    pub bidder: Pubkey,
    pub price: u64,
    pub timestamp: UnixTimestamp,
}

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
    pub current_bid_info: Option<BidInfo>,
    pub previous_bid_infos: Vec<BidInfo>,
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
            current_bid_info: None,
            previous_bid_infos: Vec::new(),
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
    ) -> Result<Option<BidInfo>, ProgramError> {
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

        let (last_bid_info, new_bid_info) = if let Some(last_bid_info) = self.current_bid_info {
            self.previous_bid_infos.insert(0, last_bid_info);
            self.previous_bid_infos.truncate(PREVIOUS_BIDDERS_LEN);

            let new_bid_info = BidInfo {
                bidder,
                price: last_bid_info.price.checked_add(raise_price).ok_or(VoilaError::MathOverflow)?,
                timestamp,
            };

            (Some(last_bid_info), new_bid_info)
        } else {
            let new_bid_info = BidInfo {
                bidder,
                price: self.base_price.checked_add(raise_price).ok_or(VoilaError::MathOverflow)?,
                timestamp,
            };

            (None, new_bid_info)
        };
        self.current_bid_info = Some(new_bid_info);

        Ok(last_bid_info)
    }

    pub fn claim(&mut self, timestamp: UnixTimestamp, owner: &Pubkey) -> ProgramResult {
        if timestamp < self.end_time {
            msg!("Auction is not end of bidding yet");
            return Err(VoilaError::InvalidBidTime.into());
        }

        if let Some(bid_info) = &self.current_bid_info {
            if &bid_info.bidder == owner {
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
    const LEN: usize = 512;
}