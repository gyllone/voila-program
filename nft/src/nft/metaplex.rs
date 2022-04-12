use metaplex_token_metadata::state::Data;
use metaplex_token_metadata::instruction::{create_metadata_accounts, create_master_edition};
use solana_program::{entrypoint::ProgramResult, account_info::AccountInfo, pubkey::Pubkey};

use crate::invoker::invoke_optionally_signed;

use super::auction::NFTAuction;
use super::{Meta, CommonNFTInfo};

impl Meta<Data> for CommonNFTInfo {
    fn metadata(&self, _mint: &Pubkey) -> Data {
        let name = format!("{}", self.name);
        // let uri = format!("{}/{}", self.uri, mint.to_string());
        let uri = "http://3.0.95.230/nft/3zhd2GY6Yx9VcY8krE2kc9heJFLdCcKdLV6BmcGLs9zn.png".to_string();

        Data {
            name,
            symbol: "VNFT".to_string(),
            uri,
            seller_fee_basis_points: 0,
            creators: None,
        }
    }
}

impl Meta<Data> for NFTAuction {
    fn metadata(&self, _mint: &Pubkey) -> Data {
        let name = format!("{}", self.name);
        let uri = "http://3.0.95.230/nft/3zhd2GY6Yx9VcY8krE2kc9heJFLdCcKdLV6BmcGLs9zn.png".to_string();

        Data {
            name,
            symbol: "VNFT".to_string(),
            uri,
            seller_fee_basis_points: 0,
            creators: None,
        }
    }
}

#[inline]
pub fn get_metaplex_metadata_account(
    program_id: &Pubkey,
    nft_mint: &Pubkey,
) -> Pubkey {
    let seeds = &[
        "metadata".as_bytes(),
        &program_id.as_ref(),
        &nft_mint.as_ref(),
    ];
    let (metadata_account, _seed) = Pubkey::find_program_address(seeds, program_id);

    metadata_account
}

#[inline]
pub fn get_metaplex_master_edition(
    program_id: &Pubkey,
    nft_mint: &Pubkey,
) -> Pubkey {
    let seeds = &[
        "metadata".as_bytes(),
        &program_id.as_ref(),
        &nft_mint.as_ref(),
        "edition".as_bytes(),
    ];
    let (master_edition_account, _seed) = Pubkey::find_program_address(seeds, &program_id);

    master_edition_account
}

#[allow(clippy::too_many_arguments)]
pub fn process_invoke_metaplex_create_metadata_accounts<'a>(
    program_account: &AccountInfo<'a>,
    metadata_account: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    mint_authority: &AccountInfo<'a>,
    user_authority: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    rent_account: &AccountInfo<'a>,
    data: Data,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    invoke_optionally_signed(
        &create_metadata_accounts(
            *program_account.key,
            *metadata_account.key,
            *mint_account.key,
            *mint_authority.key,
            *user_authority.key,
            *user_authority.key,
            data.name,
            data.symbol,
            data.uri,
            data.creators,
            data.seller_fee_basis_points,
            false,
            true,
        ),
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            user_authority.clone(),
            user_authority.clone(),
            system_program.clone(),
            rent_account.clone(),
            program_account.clone(),
        ],
        signer_seeds,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn process_invoke_metaplex_create_master_edition_accounts<'a>(
    program_account: &AccountInfo<'a>,
    metadata_account: &AccountInfo<'a>,
    edition_account: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    mint_authority: &AccountInfo<'a>,
    user_authority: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    rent_account: &AccountInfo<'a>,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    invoke_optionally_signed(
        &create_master_edition(
            *program_account.key,
            *edition_account.key,
            *mint_account.key,
            *user_authority.key,
            *mint_authority.key,
            *metadata_account.key,
            *user_authority.key,
            None,
        ),
        &[
            edition_account.clone(),
            mint_account.clone(),
            user_authority.clone(),
            mint_authority.clone(),
            user_authority.clone(),
            metadata_account.clone(),
            token_program.clone(),
            system_program.clone(),
            rent_account.clone(),
            program_account.clone(),
        ],
        signer_seeds,
    )
}