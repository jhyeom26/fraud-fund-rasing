#[cfg(test)]
mod tests {
    use casperlabs_engine_test_support::{
        internal::{ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_RUN_GENESIS_REQUEST},
        DEFAULT_ACCOUNT_ADDR,
    };
    use casperlabs_types::U512;
    #[test]
    fn draining_user_purse() {
        let install_contract_request =
            ExecuteRequestBuilder::standard(DEFAULT_ACCOUNT_ADDR, "contract.wasm", ()).build();
        let mut builder = InMemoryWasmTestBuilder::default();
        builder
            .run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST)
            .exec(install_contract_request)
            .expect_success()
            .commit();

        let account = builder
            .get_account(DEFAULT_ACCOUNT_ADDR)
            .expect("should get account");

        let account_purse = account.main_purse();
        let account_balance = builder.get_purse_balance(account_purse);
        println!("before balance:{}", account_balance);

        let fraud_contract_hash = builder
            .get_account(DEFAULT_ACCOUNT_ADDR)
            .expect("should get account")
            .named_keys()
            .get("fraud_fund_raising_proxy")
            .expect("should get fraud_fund_rasing_proxy key")
            .into_hash()
            .expect("should be hash");

        let amount = U512::from(10);
        let transfer_exec_request = ExecuteRequestBuilder::contract_call_by_hash(
            DEFAULT_ACCOUNT_ADDR,
            fraud_contract_hash,
            (amount,),
        )
        .build();
        builder
            .exec(transfer_exec_request)
            .expect_success()
            .commit();
        let account = builder
            .get_account(DEFAULT_ACCOUNT_ADDR)
            .expect("should get account");

        let account_purse = account.main_purse();
        let account_balance = builder.get_purse_balance(account_purse);
        println!("after  balance:{}", account_balance);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
