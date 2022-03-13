#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, Key};

fn make_dictionary_item_key(owner: Key) -> String {
    let preimage = owner.to_bytes().unwrap_or_revert();
    base64::encode(&preimage)
}

#[no_mangle]
pub extern "C" fn call() {
    // let key = Key::from_formatted_str(
    //     "hash-9e91c68f5e1b8c020a056f037dc669dc1d5a385ff7bf7594587fd2cefca8ff71",
    // )
    // .unwrap();

    //======================
    let key = runtime::get_named_arg("input_key");
    // provided like this
    // --session-arg "input_key:key='hash-9e91c68f5e1b8c020a056f037dc669dc1d5a385ff7bf7594587fd2cefca8ff71'"
    // or
    // --session-arg "input_key:key='account-hash-2293223427d59ebb331ac2221c3fcd1b3656a5cb72be924a6cdc9d52cdb6db0f'"
    //======================
    let dict_item_key = make_dictionary_item_key(key);

    runtime::put_key("mydicitemkey", storage::new_uref(dict_item_key).into());
}
