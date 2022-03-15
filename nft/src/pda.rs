use solana_program::pubkey::Pubkey;

const KEY_IDENTIFIER: &[u8] = "key".as_bytes();
const COMMON_NFT_IDENTIFIER: &[u8] = "commonnft".as_bytes();

#[inline]
pub fn get_key_info_pda<'a>(
    admin_authority: &'a Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, &'static [u8], &'a [u8], [u8; 1]) {
    let admin_authority_ref = admin_authority.as_ref();

    let (key, seed) = Pubkey::find_program_address(
        &[KEY_IDENTIFIER, admin_authority_ref],
        program_id,
    );

    (key, KEY_IDENTIFIER, admin_authority_ref, [seed])
}

#[inline]
pub fn get_user_key_record_pda<'a>(
    key: &'a Pubkey,
    user_authority: &'a Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, &'a [u8], &'a [u8], [u8; 1]) {
    let key_ref = key.as_ref();
    let user_authority_ref = user_authority.as_ref();

    let (key, seed) = Pubkey::find_program_address(
        &[key_ref, user_authority_ref],
        program_id,
    );

    (key, key_ref, user_authority_ref, [seed])
}

#[inline]
pub fn get_common_nft_pda<'a>(
    admin_authority: &'a Pubkey,
    name: &'a str,
    program_id: &Pubkey,
) -> (Pubkey, &'a [u8], &'a [u8], &'a [u8], [u8; 1]) {
    let admin_authority_ref = admin_authority.as_ref();
    let name_ref = name.as_bytes();

    let (key, seed) = Pubkey::find_program_address(
        &[COMMON_NFT_IDENTIFIER, admin_authority_ref, &name_ref],
        program_id,
    );

    (key, COMMON_NFT_IDENTIFIER, admin_authority_ref, name_ref, [seed])
}

#[inline]
pub fn get_common_nft_authority_pda<'a>(
    common_nft: &'a Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, &'a [u8], [u8; 1]) {
    let common_nft_ref = common_nft.as_ref();

    let (key, seed) = Pubkey::find_program_address(
        &[common_nft_ref],
        program_id,
    );

    (key, common_nft_ref, [seed])
}

#[inline]
pub fn get_common_nft_mint_pda<'a>(
    common_nft: &'a Pubkey,
    id: u16,
    program_id: &Pubkey,
) -> (Pubkey, &'a [u8], [u8; 2], [u8; 1]) {
    let common_nft_ref = common_nft.as_ref();
    let id_array = id.to_le_bytes();

    let (key, seed) = Pubkey::find_program_address(
        &[common_nft_ref, &id_array],
        program_id,
    );

    (key, common_nft_ref, id_array, [seed])
}

// #[inline]
// pub fn get_common_nft_mint_pda_by_seed(
//     common_nft: &Pubkey,
//     id: u16,
//     seed: [u8; 1],
//     program_id: &Pubkey,
// ) -> Result<Pubkey, ProgramError> {
//     let key = Pubkey::create_program_address(
//         &[
//             common_nft.as_ref(),
//             &id.to_le_bytes(),
//             &seed,
//         ],
//         program_id,
//     )?;

//     Ok(key)
// }
