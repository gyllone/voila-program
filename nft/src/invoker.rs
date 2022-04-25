use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    system_instruction,
    program_error::ProgramError,
    instruction::Instruction,
    program::{invoke, invoke_signed},
};

pub fn process_transfer<'a>(
    from_info: &AccountInfo<'a>,
    to_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    lamports: u64,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    invoke_optionally_signed(
        &system_instruction::transfer(
            &from_info.key,
            &to_info.key,
            lamports,
        ),
        &[
            from_info.clone(),
            to_info.clone(),
            system_program_info.clone(),
        ],
        signer_seeds,
    )
}

#[inline(never)]
pub fn process_optimal_create_account<'a>(
    rent_info: &AccountInfo<'a>,
    target_account_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    owner: &Pubkey,
    data_len: usize,
    signer_seeds: &[&[u8]],
    target_signer_seeds: &[&[u8]],
) -> ProgramResult {
    if target_account_info.owner == owner {
        return Ok(());
    } else if target_account_info.owner != system_program_info.key {
        return Err(ProgramError::IllegalOwner);
    }

    let required_lamports = Rent::from_account_info(rent_info)?
        .minimum_balance(data_len)
        .saturating_sub(target_account_info.lamports());

    if required_lamports > 0 {
        invoke_optionally_signed(
            &system_instruction::transfer(
                authority_info.key,
                target_account_info.key,
                required_lamports,
            ),
            &[
                authority_info.clone(),
                target_account_info.clone(),
                system_program_info.clone(),
            ],
            signer_seeds,
        )?;
    }

    invoke_optionally_signed(
        &system_instruction::allocate(target_account_info.key, data_len as u64),
        &[target_account_info.clone(), system_program_info.clone()],
        target_signer_seeds,
    )?;

    invoke_optionally_signed(
        &system_instruction::assign(target_account_info.key, owner),
        &[target_account_info.clone(), system_program_info.clone()],
        target_signer_seeds,
    )
}

/// Invoke signed unless signers seeds are empty
#[inline(always)]
pub fn invoke_optionally_signed(
    instruction: &Instruction,
    account_infos: &[AccountInfo],
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    if signer_seeds.is_empty() {
        invoke(instruction, account_infos)
    } else {
        invoke_signed(instruction, account_infos, &[signer_seeds])
    }
}
