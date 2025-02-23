use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[test]
fn test_proxy_1() {
    // Set up environment.
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, 0, 0);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(include_code!("cross_blueprint_call"));

    // Airdrop blueprint.
    executor.overwrite_package(
        Address::from_str("01bda8686d6c2fa45dce04fac71a09b54efbc8028c23aac74bc00e").unwrap(),
        include_code!("cross_blueprint_call"),
    );

    // Test the `new` function.
    let transaction1 = TransactionBuilder::new(&executor)
        .call_function(package, "Proxy1", "new", vec![], None)
        .build(vec![key])
        .unwrap();
    let receipt1 = executor.run(transaction1, true).unwrap();
    println!("{:?}\n", receipt1);
    assert!(receipt1.success);

    // Test the `get_gumball` method.
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(component, "free_token", vec![], Some(account))
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2, true).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.success);
}

#[test]
fn test_proxy_2() {
    // Set up environment.
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, 0, 0);
    let key = executor.new_public_key();
    let account = executor.new_account(key);
    let package = executor.publish_package(include_code!("cross_blueprint_call"));

    // Airdrop blueprint.
    executor.overwrite_package(
        Address::from_str("01bda8686d6c2fa45dce04fac71a09b54efbc8028c23aac74bc00e").unwrap(),
        include_code!("cross_blueprint_call"),
    );

    // Test the `new` function.
    let transaction1 = TransactionBuilder::new(&executor)
        .call_function(package, "Proxy2", "new", vec![], None)
        .build(vec![key])
        .unwrap();
    let receipt1 = executor.run(transaction1, true).unwrap();
    println!("{:?}\n", receipt1);
    assert!(receipt1.success);

    // Test the `get_gumball` method.
    let component = receipt1.component(0).unwrap();
    let transaction2 = TransactionBuilder::new(&executor)
        .call_method(component, "free_token", vec![], Some(account))
        .drop_all_bucket_refs()
        .deposit_all_buckets(account)
        .build(vec![key])
        .unwrap();
    let receipt2 = executor.run(transaction2, true).unwrap();
    println!("{:?}\n", receipt2);
    assert!(receipt2.success);
}
