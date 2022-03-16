use solana_program::{
    msg,
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

use crate::{
    invoker::process_optimal_create_account,
    nft::CommonNFTInfo,
    key::{KeyInfo, UserKeyRecord},
    Packer,
    error::VoilaError,
    token::*,
    pda::*,
    instruction::VoilaInstruction,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = VoilaInstruction::unpack(input)?;
    match instruction {
        VoilaInstruction::PurchaseKey => process_purchase_key(program_id, accounts),
        VoilaInstruction::PurchaseCommonNFT => process_purchase_common_nft(program_id, accounts),
        #[cfg(feature = "metaplex")]
        VoilaInstruction::BindCommonNFTOnMetaplex => process_bind_common_nft_on_metaplex(accounts),
        VoilaInstruction::CreateKeyInfo(receipt, price) => process_create_key_info(program_id, accounts, receipt, price),
        VoilaInstruction::CreateCommonNFT(receipt, price, max_amount, name, uri) => {
            process_create_common_nft(program_id, accounts, receipt, price, max_amount, name, uri)
        }
    }
}

#[inline(never)]
fn process_create_key_info(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    receipt: Pubkey,
    price: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let rent_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let key_info = next_account_info(account_info_iter)?;
    let admin_authority_info = next_account_info(account_info_iter)?;

    let (key, seed_1, seed_2, ref seed_3)
        = get_key_info_pda(admin_authority_info.key, program_id);
    if key_info.key != &key {
        msg!("Key info pubkey is an invalid pda pubkey");
        return Err(VoilaError::InvalidPdaPubkey.into()); 
    }

    process_optimal_create_account(
        rent_info,
        key_info,
        admin_authority_info,
        system_program_info,
        program_id,
        KeyInfo::LEN,
        &[],
        &[seed_1, seed_2, seed_3],
    )?;

    KeyInfo::new(*admin_authority_info.key, receipt, price)
        .initialize(&mut key_info.try_borrow_mut_data()?)
}

#[inline(never)]
fn process_purchase_key(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let clock = Clock::from_account_info(next_account_info(account_info_iter)?)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let key_info = next_account_info(account_info_iter)?;
    let user_record_info = next_account_info(account_info_iter)?;
    let receipt_info = next_account_info(account_info_iter)?;
    let user_authority_info = next_account_info(account_info_iter)?;

    let ki = KeyInfo::unpack(&key_info.try_borrow_data()?)?;
    if &ki.receipt != receipt_info.key {
        msg!("Receipt account in key info account is not matched with provided");
        return Err(VoilaError::UnmatchedAccounts.into());
    }

    msg!("Purchase for key, price = {}", ki.price);

    let (key, seed_1, seed_2, ref seed_3)
        = get_user_key_record_pda(key_info.key, user_authority_info.key, program_id);
    if &key != user_record_info.key {
        msg!("User key record pubkey is an invalid pda pubkey");
        return Err(VoilaError::InvalidPdaPubkey.into());
    }

    // pay for key
    **receipt_info.lamports.borrow_mut() = receipt_info
        .lamports()
        .checked_add(ki.price)
        .ok_or(VoilaError::MathOverflow)?;
    **user_authority_info.lamports.borrow_mut() = user_authority_info
        .lamports()
        .checked_sub(ki.price)
        .ok_or(VoilaError::MathOverflow)?;

    process_optimal_create_account(
        rent_info,
        user_record_info,
        user_authority_info,
        system_program_info,
        program_id,
        UserKeyRecord::LEN,
        &[],
        &[seed_1, seed_2, seed_3],
    )?;

    UserKeyRecord::new(*key_info.key, &clock, ki.price)
        .initialize(&mut user_record_info.try_borrow_mut_data()?)
}

#[inline(never)]
fn process_create_common_nft(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    receipt: Pubkey,
    price: u64,
    max_amount: u16,
    name: String,
    uri: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let rent_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let common_nft_info = next_account_info(account_info_iter)?;
    let admin_authority_info = next_account_info(account_info_iter)?;

    let (key, seed_1, seed_2, seed_3, ref seed_4)
        = get_common_nft_pda(admin_authority_info.key, &name, program_id);
    if common_nft_info.key != &key {
        msg!("Common NFT info pubkey is an invalid pda pubkey");
        return Err(VoilaError::InvalidPdaPubkey.into()); 
    }

    process_optimal_create_account(
        rent_info,
        common_nft_info,
        admin_authority_info,
        system_program_info,
        program_id,
        CommonNFTInfo::LEN,
        &[],
        &[seed_1, seed_2, seed_3, seed_4],
    )?;

    CommonNFTInfo::new(
        *admin_authority_info.key,
        receipt,
        common_nft_info.key,
        program_id,
        price,
        max_amount,
        name,
        uri,
    ).initialize(&mut common_nft_info.try_borrow_mut_data()?)
}

#[inline(never)]
fn process_purchase_common_nft(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let rent_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let spl_associated_program_info = next_account_info(account_info_iter)?;
    let common_nft_info = next_account_info(account_info_iter)?;
    let common_nft_authority_info = next_account_info(account_info_iter)?;
    let user_nft_mint_info = next_account_info(account_info_iter)?;
    let user_nft_account_info = next_account_info(account_info_iter)?;
    let receipt_info = next_account_info(account_info_iter)?;
    let user_authority_info = next_account_info(account_info_iter)?;

    let mut nft_info = CommonNFTInfo::unpack(&common_nft_info.try_borrow_data()?)?;
    if &nft_info.receipt != receipt_info.key {
        msg!("Receipt account in common NFT info is not matched with provided");
        return Err(VoilaError::UnmatchedAccounts.into());
    }
    if &nft_info.pda_authority != common_nft_authority_info.key {
        msg!("Authority account in common NFT info is not matched with provided");
        return Err(VoilaError::UnmatchedAccounts.into());
    }

    msg!("Purchase for common NFT, name = {}, price = {}, current amount = {}, max amount = {}",
        nft_info.name, nft_info.price, nft_info.current_amount, nft_info.max_amount);

    if nft_info.current_amount >= nft_info.max_amount {
        return Err(VoilaError::NFTEndOfSale.into());
    }

    let (key, seed_1, ref seed_2, ref seed_3)
        = get_common_nft_mint_pda(common_nft_info.key, nft_info.current_amount, program_id);
    if &key != user_nft_mint_info.key {
        msg!("User common NFT mint pubkey is an invalid pda pubkey");
        return Err(VoilaError::InvalidPdaPubkey.into());
    }

    // pay for nft
    **receipt_info.lamports.borrow_mut() = receipt_info
        .lamports()
        .checked_add(nft_info.price)
        .ok_or(VoilaError::MathOverflow)?;
    **user_authority_info.lamports.borrow_mut() = user_authority_info
        .lamports()
        .checked_sub(nft_info.price)
        .ok_or(VoilaError::MathOverflow)?;

    process_init_token_mint(
        rent_info,
        user_nft_mint_info,
        user_authority_info,
        token_program_info,
        system_program_info,
        common_nft_authority_info.key,
        0,
        &[],
        &[seed_1, seed_2, seed_3],
    )?;

    process_create_associated_token_account(
        rent_info,
        user_nft_mint_info,
        user_nft_account_info,
        user_authority_info,
        user_authority_info,
        token_program_info,
        system_program_info,
        spl_associated_program_info,
        &[],
    )?;

    process_token_mint_to(
        token_program_info,
        user_nft_mint_info,
        user_nft_account_info,
        common_nft_authority_info,
        &nft_info.authority_signer_seeds(common_nft_info.key),
        1,
    )?;

    nft_info.current_amount += 1;
    nft_info.pack(&mut common_nft_info.try_borrow_mut_data()?)
}

#[cfg(feature = "metaplex")]
fn process_bind_common_nft_on_metaplex(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let rent_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let metaplex_program_info = next_account_info(account_info_iter)?;
    let common_nft_info = next_account_info(account_info_iter)?;
    let common_nft_authority_info = next_account_info(account_info_iter)?;
    let user_nft_mint_info = next_account_info(account_info_iter)?;
    let metadata_account_info = next_account_info(account_info_iter)?;
    let master_edition_account_info = next_account_info(account_info_iter)?;
    let user_authority_info = next_account_info(account_info_iter)?;

    use crate::nft::{metaplex::{process_invoke_metaplex_create_metadata_accounts, process_invoke_metaplex_create_master_edition_accounts}, Meta};

    let nft_info = CommonNFTInfo::unpack(&common_nft_info.try_borrow_data()?)?;
    if &nft_info.pda_authority != common_nft_authority_info.key {
        msg!("Authority account in common NFT info is not matched with provided");
        return Err(VoilaError::UnmatchedAccounts.into());
    }
    let signer_seeds = &nft_info.authority_signer_seeds(common_nft_info.key);
    let data = nft_info.metadata(user_nft_mint_info.key);

    process_invoke_metaplex_create_metadata_accounts(
        metaplex_program_info,
        metadata_account_info,
        user_nft_mint_info,
        common_nft_authority_info,
        user_authority_info,
        system_program_info,
        rent_info,
        data,
        signer_seeds,
    )?;

    process_invoke_metaplex_create_master_edition_accounts(
        metaplex_program_info,
        metadata_account_info,
        master_edition_account_info,
        user_nft_mint_info,
        common_nft_authority_info,
        user_authority_info,
        token_program_info,
        system_program_info,
        rent_info,
        signer_seeds,
    )
}