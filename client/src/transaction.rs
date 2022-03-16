use solana_program::{pubkey::Pubkey, hash::Hash};
use solana_sdk::{signature::Keypair, transaction::Transaction, signer::Signer};

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
    Transaction::new_signed_with_payer(
        &[
            voila_nft::instruction::purchase_common_nft(
                nft_info,
                receipt,
                user_authority.pubkey(),
                nft_id,
            ),
        ],
        Some(&user_authority.pubkey()),
        &[user_authority],
        blockhash,
    )
}