#[allow(missing_docs)]
pub mod key;
pub mod error;
pub mod entrypoint;
pub mod invoker;
pub mod instruction;
pub mod nft;
pub mod token;
pub mod pda;
pub mod processor;

pub use solana_program;

solana_program::declare_id!("SF1EUyhLKzxaDtV7zHMtuVXpaqsmDB8GEkNuH2235tf");

use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    entrypoint::ProgramResult,
    program_pack::IsInitialized,
    program_error::ProgramError,
};
use crate::error::VoilaError;

pub trait Data: Sized {
    fn to_vec(self) -> Vec<u8>;
}

pub trait Updater<P> {
    fn can_update(&self, param: &P) -> bool;

    fn update_unchecked(&mut self, param: P);
    
    fn update(&mut self, param: P) -> ProgramResult {
        if self.can_update(&param) {
            self.update_unchecked(param);

            Ok(())
        } else {
            Err(VoilaError::InvalidParam.into())
        }
    }
}

pub trait Packer: IsInitialized + BorshSerialize + BorshDeserialize {
    const LEN: usize;

    fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        let account: Self = BorshDeserialize::deserialize(&mut data.as_ref())?;
        if account.is_initialized() {
            Ok(account)
        } else {
            Err(VoilaError::NotInitialized.into())
        }
    }

    fn pack(self, data: &mut [u8]) -> ProgramResult {
        self.serialize(&mut data.as_mut())?;

        Ok(())
    }

    fn initialize(self, data: &mut [u8]) -> ProgramResult {
        if data.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        let account: Self = BorshDeserialize::deserialize(&mut data.as_ref())?;
        if account.is_initialized() {
            Err(VoilaError::AlreadyInitialized.into())
        } else {
            self.pack(data)
        }
    }
}
