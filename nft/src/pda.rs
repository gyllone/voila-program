use solana_program::pubkey::Pubkey;

const KEY_IDENTIFIER: &[u8] = "key".as_bytes();
const COMMON_NFT_IDENTIFIER: &[u8] = "commonnft".as_bytes();
const NFT_AUCTION_IDENTIFIER: &[u8] = "auction".as_bytes();

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

#[inline]
pub fn get_nft_auction_pda<'a>(
    admin_authority: &'a Pubkey,
    sn: u16,
    program_id: &Pubkey,
) -> (Pubkey, &'a [u8], &'a [u8], [u8; 2], [u8; 1]) {
    let admin_authority_ref = admin_authority.as_ref();
    let sn_array = sn.to_le_bytes();

    let (key, seed) = Pubkey::find_program_address(
        &[NFT_AUCTION_IDENTIFIER, admin_authority_ref, &sn_array],
        program_id,
    );

    (key, NFT_AUCTION_IDENTIFIER, admin_authority_ref, sn_array, [seed])
}

#[inline]
pub fn get_nft_auction_authority_pda<'a>(
    nft_auction: &'a Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, &'a [u8], [u8; 1]) {
    let nft_auction_ref = nft_auction.as_ref();

    let (key, seed) = Pubkey::find_program_address(
        &[nft_auction_ref],
        program_id,
    );

    (key, nft_auction_ref, [seed])
}

#[inline]
pub fn get_auction_nft_mint_pda<'a>(
    authority: &'a Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, &'a [u8], [u8; 1]) {
    let authority_ref = authority.as_ref();

    let (key, seed) = Pubkey::find_program_address(
        &[authority_ref],
        program_id,
    );

    (key, authority_ref, [seed])
}
