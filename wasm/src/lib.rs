#![allow(missing_docs)]

use js_sys::{Uint8Array, Array};
use solana_program::{program_pack::Pack, pubkey::Pubkey};
use spl_token::state::Account;
use wasm_bindgen::{JsValue, prelude::*};
use voila_nft::{Packer, key::{KeyInfo, UserKeyRecord}, nft::{CommonNFTInfo, auction::NFTAuction}};

#[wasm_bindgen]
pub fn get_key_info(key_info_data: Uint8Array) -> JsValue {
    console_error_panic_hook::set_once();

    let key_info_data = key_info_data.to_vec();
    let key_info = KeyInfo::unpack(&key_info_data)
        .expect("key info data can not unpack");

    JsValue::from_serde(&key_info).expect("serde serialize")
}

#[wasm_bindgen]
pub fn get_key_record(key_record_data: Uint8Array) -> JsValue {
    console_error_panic_hook::set_once();

    let key_record_data = key_record_data.to_vec();
    let key_record = UserKeyRecord::unpack(&key_record_data)
        .expect("key record data can not unpack");

    JsValue::from_serde(&key_record).expect("serde serialize")
}

#[wasm_bindgen]
pub fn get_common_nft_info(common_nft_data: Uint8Array) -> JsValue {
    console_error_panic_hook::set_once();

    let common_nft_data = common_nft_data.to_vec();
    let common_nft = CommonNFTInfo::unpack(&common_nft_data)
        .expect("common nft data can not unpack");

    JsValue::from_serde(&common_nft).expect("serde serialize")
}

#[wasm_bindgen]
pub fn get_nft_auction(nft_auction_data: Uint8Array) -> JsValue {
    console_error_panic_hook::set_once();

    let nft_auction_data = nft_auction_data.to_vec();
    let nft_auction = NFTAuction::unpack(&nft_auction_data)
        .expect("nft auction data can not unpack");

    JsValue::from_serde(&nft_auction).expect("serde serialize")
}

#[wasm_bindgen]
pub fn get_user_nft_tokens(owner: Pubkey, nft_tokens: Array) -> JsValue {
    console_error_panic_hook::set_once();

    let owning_nfts = nft_tokens
        .iter()
        .filter_map(|account| {
            let account = Uint8Array::from(account).to_vec();
            if let Ok(account) = Account::unpack(&account) {
                if account.owner == owner {
                    Some(account.mint.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    JsValue::from_serde(&owning_nfts).expect("serde serialize")
}