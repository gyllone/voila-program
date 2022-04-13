use solana_program::{pubkey::Pubkey, hash::Hash};
use solana_sdk::{signature::Keypair, transaction::Transaction, signer::Signer};
use voila_nft::pda::get_common_nft_mint_pda;

pub fn do_create_key_info(
    admin_authority: &Keypair,
    receipt: Pubkey,
    price: u64,
    blockhash: Hash,
) -> Transaction {
    Transaction::new_signed_with_payer(
        &[
            voila_nft::instruction::create_key_info(
                admin_authority.pubkey(),
                receipt,
                price,
            ),
        ],
        Some(&admin_authority.pubkey()),
        &[admin_authority],
        blockhash,
    )
}

pub fn do_create_nft_auction(
    admin_authority: &Keypair,
    sn: u16,
    start_time: i64,
    end_time: i64,
    base_price: u64,
    price_raise: u64,
    name: String,
    uri: String,
    blockhash: Hash,
) -> Transaction {
    Transaction::new_signed_with_payer(
        &[
            voila_nft::instruction::create_nft_auction(
                admin_authority.pubkey(),
                sn,
                start_time,
                end_time,
                base_price,
                price_raise,
                name,
                uri,
            ),
        ],
        Some(&admin_authority.pubkey()),
        &[admin_authority],
        blockhash,
    )
}

pub fn do_create_common_nft(
    admin_authority: &Keypair,
    receipt: Pubkey,
    price: u64,
    max_amount: u16,
    name: String,
    uri: String,
    blockhash: Hash,
) -> Transaction {
    Transaction::new_signed_with_payer(
        &[
            voila_nft::instruction::create_common_nft(
                admin_authority.pubkey(),
                receipt,
                price,
                max_amount,
                name,
                uri,
            ),
        ],
        Some(&admin_authority.pubkey()),
        &[admin_authority],
        blockhash,
    )
}

pub fn do_purchase_key(
    user_authority: &Keypair,
    key_info: Pubkey,
    receipt: Pubkey,
    blockhash: Hash,
) -> Transaction {
    Transaction::new_signed_with_payer(
        &[
            voila_nft::instruction::purchase_key(
                key_info,
                receipt,
                user_authority.pubkey(),
            ),
        ],
        Some(&user_authority.pubkey()),
        &[user_authority],
        blockhash,
    )
}

pub fn do_purchase_common_nft(
    user_authority: &Keypair,
    nft_info: Pubkey,
    receipt: Pubkey,
    nft_id: u16,
    blockhash: Hash,
) -> Transaction {
    let (nft_mint, _, _, _) = get_common_nft_mint_pda(&nft_info, nft_id, &voila_nft::ID);

    Transaction::new_signed_with_payer(
        &[
            voila_nft::instruction::purchase_common_nft(
                nft_info,
                receipt,
                user_authority.pubkey(),
                nft_id,
            ),
            voila_nft::instruction::bind_common_nft_on_metaplex(
                nft_info,
                nft_mint,
                user_authority.pubkey(),
            ),
        ],
        Some(&user_authority.pubkey()),
        &[user_authority],
        blockhash,
    )
}