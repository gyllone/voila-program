use spl_token::state::Mint;
use spl_associated_token_account::create_associated_token_account;
use solana_program::{
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::Pack,
    account_info::AccountInfo,
    pubkey::Pubkey,
};

use crate::invoker::{invoke_optionally_signed, process_optimal_create_account};

#[allow(clippy::too_many_arguments)]
pub fn process_create_associated_token_account<'a>(
    rent_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    user_token_account_info: &AccountInfo<'a>,
    payer_authority_info: &AccountInfo<'a>,
    owner_authority_info: &AccountInfo<'a>,
    token_program_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    spl_associated_program_info: &AccountInfo<'a>,
    payer_signer_seeds: &[&[u8]],
) -> ProgramResult {
    if user_token_account_info.owner == system_program_info.key {
        invoke_optionally_signed(
            &create_associated_token_account(
                payer_authority_info.key,
                owner_authority_info.key,
                mint_info.key,
            ),
            &[
                payer_authority_info.clone(),
                owner_authority_info.clone(),
                user_token_account_info.clone(),
                mint_info.clone(),
                system_program_info.clone(),
                token_program_info.clone(),
                rent_info.clone(),
                spl_associated_program_info.clone(),
            ],
            payer_signer_seeds,
        )
    } else if user_token_account_info.owner != token_program_info.key {
        Err(ProgramError::IllegalOwner)
    } else {
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
pub fn process_init_token_mint<'a>(
    rent_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    payer_authority_info: &AccountInfo<'a>,
    token_program_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    owner: &Pubkey,
    decimal: u8,
    payer_signer_seeds: &[&[u8]],
    mint_signer_seeds: &[&[u8]],
) -> ProgramResult {
    process_optimal_create_account(
        rent_info,
        mint_info,
        payer_authority_info,
        system_program_info,
        token_program_info.key,
        Mint::LEN,
        payer_signer_seeds,
        mint_signer_seeds,
    )?;

    invoke_optionally_signed(
        &spl_token::instruction::initialize_mint(
            token_program_info.key,
            mint_info.key,
            owner,
            None,
            decimal,
        )?,
        &[
            mint_info.clone(),
            rent_info.clone(),
            token_program_info.clone(),
        ],
        &[],
    )
}

#[allow(clippy::too_many_arguments)]
pub fn process_token_mint_to<'a>(
    token_program: &AccountInfo<'a>,
    token_mint: &AccountInfo<'a>,
    token_account: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    signer_seeds: &[&[u8]],
    amount: u64,
) -> ProgramResult {
    invoke_optionally_signed(
        &spl_token::instruction::mint_to(
            token_program.key,
            token_mint.key,
            token_account.key,
            authority.key,
            &[],
            amount,
        )?,
        &[
            token_mint.clone(),
            token_account.clone(),
            authority.clone(),
            token_program.clone(),
        ],
        signer_seeds,
    )
}
