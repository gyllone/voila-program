mod transaction;

use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{signature::Keypair, commitment_config::{CommitmentConfig, CommitmentLevel}, pubkey, signer::Signer};
use voila_nft::{nft::auction::NFTAuction, Packer};

const DEVNET: &str = "https://api.devnet.solana.com";
const MAINNET: &str = "https://api.mainnet-beta.solana.com";

const USER_KEYPAIR: &str = "25VtdefYWzk4fvyfAg3RzSrhwmy4HhgPyYcxetmHRmPrkCsDqSJw8Jav7tWCXToV6e1L7nGxhyEDnWYVsDHUgiZ7";
const ADMIN_KEYPAIR: &str = "pEyHAq7jGET5KcmTw4Rh4kPu7Auec6Nc16TRzoXuZyGXVyqD41zqh2WRBjq9fSKChszMS5iHa1m14mFhpmu1LfM";

const KEY_PUBKEY: Pubkey = pubkey!("3jTDEb5b21xGED9mdrpU2ipf2RqrF73ZXsBfW3ombe5A");
const COMMON_NFT_PUBKEY: Pubkey = pubkey!("SH6YNRpuQsn6S7imChn9t7yJjBy9WnTofr2bMSpwUBs"); 
const COMMON_PRIMARY: Pubkey = pubkey!("vskpF4rYHp36N8uyuq5WTFA8peZUTfAxYXFyHvxK6Cy");
const COMMON_SENIOR: Pubkey = pubkey!("3jDyTJHfYAS9WEhju774esX92urm4tWT5HhQ6qwsAjY9");
const NFT_AUCTION: Pubkey = pubkey!("2sd16nzRhXEDq3FU1ux4QqGpEM1716Ecyjvw2NyVuTYm");

fn main() {
    let client = RpcClient::new_with_commitment(DEVNET, CommitmentConfig {
        commitment: CommitmentLevel::Processed,
    });

    let blockhash = client.get_latest_blockhash().unwrap();
    let admin = Keypair::from_base58_string(ADMIN_KEYPAIR);
    let user = Keypair::from_base58_string(USER_KEYPAIR);

    // let tx = transaction::do_create_key_info(&admin, admin.pubkey(), 33300000, blockhash);

    // let tx = transaction::do_purchase_key(&user, KEY_PUBKEY, admin.pubkey(), blockhash);

    // let tx = transaction::do_create_common_nft(
    //     &admin,
    //     admin.pubkey(),
    //     110000000,
    //     205,
    //     "primary".to_string(),
    //     "https://voila.com".to_string(),
    //     blockhash,
    // );

    // let tx = transaction::do_create_common_nft(
    //     &admin,
    //     admin.pubkey(),
    //     990000000,
    //     100,
    //     "senior".to_string(),
    //     "https://voila.com".to_string(),
    //     blockhash,
    // );

    // let tx = transaction::do_create_nft_auction(
    //     &admin,
    //     3,
    //     1650616200,
    //     1750789000,
    //     1_000_000_000,
    //     100_000_000,
    //     "auction".to_string(),
    //     "https://voila.com".to_string(),
    //     blockhash,
    // );

    // let data = client.get_account_data(&Pubkey::from_str("GAzSgn6gcEcGibG18GgLzYBkqkaCVSvZ3GcJoBoU5XRu").unwrap()).unwrap();
    // println!("{:?}", &data);
    // let auction = NFTAuction::unpack(&data).unwrap();
    // println!("{:?}", auction.pda_authority);

    // let tx = transaction::do_purchase_common_nft(
    //     &user,
    //     COMMON_NFT_PUBKEY,
    //     admin.pubkey(),
    //     5,
    //     blockhash,
    // );

    let tx = transaction::do_bid_in_nft_auction(
        &user,
        NFT_AUCTION,
        None,
        100_000_000,
        blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx).unwrap();
    println!("sig {}", sig);

    // let res = client.simulate_transaction(&tx).unwrap();
    // println!("{:?}", res.value);
}
