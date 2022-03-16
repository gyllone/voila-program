mod transaction;

use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{signature::Keypair, signer::Signer, commitment_config::{CommitmentConfig, CommitmentLevel}, pubkey};

const DEVNET: &str = "https://api.devnet.solana.com";
const MAINNET: &str = "https://api.mainnet-beta.solana.com";

const USER_KEYPAIR: &str = "25VtdefYWzk4fvyfAg3RzSrhwmy4HhgPyYcxetmHRmPrkCsDqSJw8Jav7tWCXToV6e1L7nGxhyEDnWYVsDHUgiZ7";
const ADMIN_KEYPAIR: &str = "pEyHAq7jGET5KcmTw4Rh4kPu7Auec6Nc16TRzoXuZyGXVyqD41zqh2WRBjq9fSKChszMS5iHa1m14mFhpmu1LfM";

const KEY_PUBKEY: Pubkey = pubkey!("3jTDEb5b21xGED9mdrpU2ipf2RqrF73ZXsBfW3ombe5A");

fn main() {
    let client = RpcClient::new_with_commitment(DEVNET, CommitmentConfig {
        commitment: CommitmentLevel::Processed,
    });

    let blockhash = client.get_latest_blockhash().unwrap();
    let admin = Keypair::from_base58_string(ADMIN_KEYPAIR);
    let user = Keypair::from_base58_string(USER_KEYPAIR);

    // let tx = transaction::do_create_key_info(&admin, admin.pubkey(), 33300000, blockhash);

    let tx = transaction::do_purchase_key(&user, KEY_PUBKEY, admin.pubkey(), blockhash);

    let sig = client.send_and_confirm_transaction(&tx).unwrap();
    println!("sig {}", sig);
}
