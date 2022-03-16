use std::convert::TryInto;
use solana_program::{
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
    msg,
    instruction::{Instruction, AccountMeta},
    system_program,
    sysvar,
};
use spl_associated_token_account::get_associated_token_address;

use crate::{error::VoilaError, pda::*, ID};

#[derive(Debug, PartialEq)]
pub enum VoilaInstruction {
    // 0 ~ 127 user
    PurchaseKey,
    PurchaseCommonNFT,
    #[cfg(feature = "metaplex")]
    BindCommonNFTOnMetaplex,
    // 128 ~ admin
    CreateKeyInfo(Pubkey, u64),
    CreateCommonNFT(Pubkey, u64, u16, String, String),
}

impl VoilaInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(VoilaError::InstructionUnpackError)?;
        Ok(match tag {
            0 => Self::PurchaseKey,
            1 => Self::PurchaseCommonNFT,
            #[cfg(feature = "metaplex")]
            2 => Self::BindCommonNFTOnMetaplex,
            128 => {
                let (receipt, rest) = Self::unpack_pubkey(rest)?;
                let (price, _rest) = Self::unpack_u64(rest)?;
                Self::CreateKeyInfo(receipt, price)
            },
            129 => {
                let (receipt, rest) = Self::unpack_pubkey(rest)?;
                let (price, rest) = Self::unpack_u64(rest)?;
                let (max_amount, rest) = Self::unpack_u16(rest)?;
                let (name, rest) = Self::unpack_string(rest)?;
                let (uri, _rest) = Self::unpack_string(rest)?;
                Self::CreateCommonNFT(receipt, price, max_amount, name, uri)
            }
            _ => return Err(VoilaError::InstructionUnpackError.into()),
        })
    }

    pub fn pack(self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            Self::PurchaseKey => buf.push(0),
            Self::PurchaseCommonNFT => buf.push(1),
            #[cfg(feature = "metaplex")]
            Self::BindCommonNFTOnMetaplex => buf.push(2),
            Self::CreateKeyInfo(receipt, price) => {
                buf.push(128);
                buf.extend_from_slice(&receipt.as_ref());
                buf.extend_from_slice(&price.to_le_bytes());
            }
            Self::CreateCommonNFT(receipt, price, max_amount, name, uri) => {
                buf.push(129);
                buf.extend_from_slice(&receipt.as_ref());
                buf.extend_from_slice(&price.to_le_bytes());
                buf.extend_from_slice(&max_amount.to_le_bytes());

                let name_data = name.as_bytes();
                buf.push(name_data.len() as u8);
                buf.extend_from_slice(name_data);

                let uri_data = uri.as_bytes();
                buf.push(uri_data.len() as u8);
                buf.extend_from_slice(uri_data);
            }
        }

        buf
    }
    
    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() < PUBKEY_BYTES {
            msg!("Pubkey cannot be unpacked");
            return Err(VoilaError::InstructionUnpackError.into());
        }
        let (key, rest) = input.split_at(PUBKEY_BYTES);
        let pk = Pubkey::new(key);
        Ok((pk, rest))
    }

    fn unpack_string(input: &[u8]) -> Result<(String, &[u8]), ProgramError> {
        let (len, rest) = input.split_first().ok_or_else(|| {
            msg!("String cannot be unpacked");
            VoilaError::InstructionUnpackError
        })?;
        let (data, rest) = rest.split_at(*len as usize);
        let s = String::from_utf8(data.to_vec())
            .map_err(|_| VoilaError::InstructionUnpackError)?;

        Ok((s, rest))
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() < 8 {
            msg!("u64 cannot be unpacked");
            return Err(VoilaError::InstructionUnpackError.into());
        }
        let (amount, rest) = input.split_at(8);
        let amount = amount
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(VoilaError::InstructionUnpackError)?;
        Ok((amount, rest))
    }

    fn unpack_u16(input: &[u8]) -> Result<(u16, &[u8]), ProgramError> {
        if input.len() < 2 {
            msg!("u16 cannot be unpacked");
            return Err(VoilaError::InstructionUnpackError.into());
        }
        let (amount, rest) = input.split_at(2);
        let amount = amount
            .get(..2)
            .and_then(|slice| slice.try_into().ok())
            .map(u16::from_le_bytes)
            .ok_or(VoilaError::InstructionUnpackError)?;
        Ok((amount, rest))
    }
}

pub fn create_key_info(
    admin_authority: Pubkey,
    receipt: Pubkey,
    price: u64,
) -> Instruction {
    let (key_info, _, _, _) = get_key_info_pda(&admin_authority, &ID);

    Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(key_info, false),
            AccountMeta::new(admin_authority, true),
        ],
        data: VoilaInstruction::CreateKeyInfo(receipt, price).pack(),
    }
}

pub fn purchase_key(
    key_info: Pubkey,
    receipt: Pubkey,
    user_authority: Pubkey,
) -> Instruction {
    let (user_key_record, _, _, _) = get_user_key_record_pda(&key_info, &user_authority, &ID);

    Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(key_info, false),
            AccountMeta::new(user_key_record, false),
            AccountMeta::new(receipt, false),
            AccountMeta::new(user_authority, true),
        ],
        data: VoilaInstruction::PurchaseKey.pack(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_common_nft(
    admin_authority: Pubkey,
    receipt: Pubkey,
    price: u64,
    max_amount: u16,
    name: String,
    uri: String,
) -> Instruction {
    let (nft_info, _, _, _, _) = get_common_nft_pda(&admin_authority, &name, &ID);

    Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(nft_info, false),
            AccountMeta::new(admin_authority, true),
        ],
        data: VoilaInstruction::CreateCommonNFT(receipt, price, max_amount, name, uri).pack(),
    }
}

pub fn purchase_common_nft(
    nft_info: Pubkey,
    receipt: Pubkey,
    user_authority: Pubkey,
    nft_id: u16,
) -> Instruction {
    let (nft_authority, _, _) = get_common_nft_authority_pda(&nft_info, &ID);
    let (nft_mint, _, _, _) = get_common_nft_mint_pda(&nft_info, nft_id, &ID);
    let nft_account = get_associated_token_address(&user_authority, &nft_mint);

    Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new(nft_info, false),
            AccountMeta::new_readonly(nft_authority, false),
            AccountMeta::new(nft_mint, false),
            AccountMeta::new(nft_account, false),
            AccountMeta::new(receipt, false),
            AccountMeta::new(user_authority, true),
        ],
        data: VoilaInstruction::PurchaseCommonNFT.pack(),
    }
}

#[cfg(feature = "metaplex")]
pub fn bind_common_nft_on_metaplex(
    nft_info: Pubkey,
    nft_mint: Pubkey,
    user_authority: Pubkey,
) -> Instruction {
    use crate::nft::metaplex::{get_metaplex_metadata_account, get_metaplex_master_edition};

    let (nft_authority, _, _) = get_common_nft_authority_pda(&nft_info, &ID);
    let metadata = get_metaplex_metadata_account(&metaplex_token_metadata::ID, &nft_mint);
    let master_edition = get_metaplex_master_edition(&metaplex_token_metadata::ID, &nft_mint);

    Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(metaplex_token_metadata::ID, false),
            AccountMeta::new_readonly(nft_info, false),
            AccountMeta::new_readonly(nft_authority, false),
            AccountMeta::new(nft_mint, false),
            AccountMeta::new(metadata, false),
            AccountMeta::new(master_edition, false),
            AccountMeta::new(user_authority, true),
        ],
        data: VoilaInstruction::BindCommonNFTOnMetaplex.pack(),
    }
}