//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    msg,
    decode_error::DecodeError,
    program_error::{ProgramError, PrintProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the TokenLending program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum VoilaError {
    #[error("Account is already initialized")]
    AlreadyInitialized,
    #[error("Accounts are not matched")]
    UnmatchedAccounts,
    #[error("NFT is end of sale")]
    NFTEndOfSale,
    #[error("NFT bid price is loo slow")]
    InvalidBidPrice,
    #[error("NFT can not bid now")]
    InvalidBidTime,
    #[error("NFT can not claim")]
    NFTCannotClaim,
    #[error("Pda pubkey is invalid")]
    InvalidPdaPubkey,
    #[error("Invalid program id")]
    InvalidProgramId,
    #[error("Input account owner is not the program address")]
    InvalidAccountOwner,
    #[error("Authority is invalid")]
    InvalidAuthority,
    #[error("Invalid update param")]
    InvalidParam,
    #[error("Failed to unpack instruction data")]
    InstructionUnpackError,
    #[error("Math operation overflow")]
    MathOverflow,
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,
    #[error("Account is not initialized")]
    NotInitialized,
}

impl From<VoilaError> for ProgramError {
    fn from(e: VoilaError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for VoilaError {
    fn type_of() -> &'static str {
        "Voila Error"
    }
}

impl PrintProgramError for VoilaError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        msg!(self.to_string().as_str());
    }
}
