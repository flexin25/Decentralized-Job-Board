#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Env, String};

#[test]
fn test_add_and_get_product() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Add a product
    client.add_product(
        &String::from_str(&env, "PROD-001"),
        &String::from_str(&env, "Factory A"),
    );

    // Get the product
    let product = client.get_product(&String::from_str(&env, "PROD-001"));
    assert_eq!(product.product_id, String::from_str(&env, "PROD-001"));
    assert_eq!(product.origin, String::from_str(&env, "Factory A"));
    assert_eq!(product.status, String::from_str(&env, "Created"));
}

#[test]
fn test_update_status() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Add a product
    client.add_product(
        &String::from_str(&env, "PROD-002"),
        &String::from_str(&env, "Factory B"),
    );

    // Update status to Shipped
    client.update_status(
        &String::from_str(&env, "PROD-002"),
        &String::from_str(&env, "Shipped"),
    );

    // Verify status changed
    let product = client.get_product(&String::from_str(&env, "PROD-002"));
    assert_eq!(product.status, String::from_str(&env, "Shipped"));

    // Update status to Delivered
    client.update_status(
        &String::from_str(&env, "PROD-002"),
        &String::from_str(&env, "Delivered"),
    );

    let product = client.get_product(&String::from_str(&env, "PROD-002"));
    assert_eq!(product.status, String::from_str(&env, "Delivered"));
}

#[test]
#[should_panic(expected = "product already exists")]
fn test_duplicate_product() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    client.add_product(
        &String::from_str(&env, "PROD-003"),
        &String::from_str(&env, "Factory C"),
    );

    // Try to add same product again - should panic
    client.add_product(
        &String::from_str(&env, "PROD-003"),
        &String::from_str(&env, "Factory D"),
    );
}

#[test]
#[should_panic(expected = "product not found")]
fn test_get_nonexistent_product() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    client.get_product(&String::from_str(&env, "NONEXISTENT"));
}
