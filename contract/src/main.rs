#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

extern crate alloc;

use alloc::{collections::BTreeMap, string::String};

use casperlabs_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casperlabs_types::{ApiError, ContractRef, Key, URef, U512};

const PROXY_CONTRACT_NAME: &str = "fraud_fund_raising_proxy";
const CONTRACT_NAME: &str = "fraud_fund_raising";
const CONTRACT_PURSE_NAME: &str = "contract_purse";

#[repr(u16)]
enum ContractError {
    FailToGetBalance = 1,
    FailToLookupPurse = 2,
    UnexpectedKey = 3,
    ContractNotFound = 4,
}

impl From<ContractError> for ApiError {
    fn from(error: ContractError) -> ApiError {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn fraud_fund_raising_proxy() {
    let amount: U512 = runtime::get_arg(0)
        .unwrap_or_revert_with(ApiError::MissingArgument)
        .unwrap_or_revert_with(ApiError::InvalidArgument);
    let requester_purse = account::get_main_purse();
    let contract_hash = match runtime::get_key(CONTRACT_NAME)
        .unwrap_or_revert_with(ContractError::ContractNotFound)
    {
        Key::Hash(hash) => ContractRef::Hash(hash),
        _ => runtime::revert(ContractError::UnexpectedKey),
    };
    runtime::call_contract(contract_hash, (amount, requester_purse))
}

#[no_mangle]
pub extern "C" fn fraud_fund_raising() {
    let _amount: U512 = runtime::get_arg(0)
        .unwrap_or_revert_with(ApiError::MissingArgument)
        .unwrap_or_revert_with(ApiError::InvalidArgument);
    let requester_purse: URef = runtime::get_arg(1)
        .unwrap_or_revert_with(ApiError::MissingArgument)
        .unwrap_or_revert_with(ApiError::InvalidArgument);
    let contract_purse = {
        match runtime::get_key(CONTRACT_PURSE_NAME)
            .unwrap_or_revert_with(ContractError::FailToLookupPurse)
        {
            Key::URef(uref) => uref,
            _ => runtime::revert(ContractError::UnexpectedKey),
        }
    };
    let transferred_amount =
        system::get_balance(requester_purse).unwrap_or_revert_with(ContractError::FailToGetBalance);

    system::transfer_from_purse_to_purse(requester_purse, contract_purse, transferred_amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn call() {
    // install fraud_fund_raise
    let contract_hash = {
        let contract_purse = system::create_purse();
        let mut named_keys = BTreeMap::new();
        named_keys.insert(String::from(CONTRACT_PURSE_NAME), contract_purse.into());
        storage::store_function_at_hash(CONTRACT_NAME, named_keys)
    };

    // install fraud_fund_raise_proxy
    let proxy_contract_hash =
        storage::store_function_at_hash(PROXY_CONTRACT_NAME, Default::default());

    runtime::put_key(CONTRACT_NAME, contract_hash.into());
    runtime::put_key(PROXY_CONTRACT_NAME, proxy_contract_hash.into());
}
