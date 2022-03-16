mod tx;

use solana_sdk::{signature::Keypair, signer::Signer};

const ADMIN_KEYPAIR: &str = "";

fn main() {
    let keypair = Keypair::new();
    println!("{}", keypair.to_base58_string());
    println!("{}", keypair.pubkey());
}
