#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Map, String};

#[contract]
pub struct Contract;

#[derive(Clone)]
#[contracttype]
pub struct Product {
    pub product_id: String,
    pub origin: String,
    pub status: String,
}

#[contractimpl]
impl Contract {
    pub fn add_product(env: Env, product_id: String, origin: String) {
        let key = symbol_short!("products");
        let mut products: Map<String, Product> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        assert!(
            !products.contains_key(product_id.clone()),
            "product already exists"
        );

        let product = Product {
            product_id: product_id.clone(),
            origin,
            status: String::from_str(&env, "Created"),
        };
        products.set(product_id, product);
        env.storage().instance().set(&key, &products);
    }

    pub fn update_status(env: Env, product_id: String, new_status: String) {
        let key = symbol_short!("products");
        let mut products: Map<String, Product> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let mut product = products.get(product_id.clone()).expect("product not found");
        product.status = new_status;
        products.set(product_id, product);
        env.storage().instance().set(&key, &products);
    }

    pub fn get_product(env: Env, product_id: String) -> Product {
        let key = symbol_short!("products");
        let products: Map<String, Product> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        products.get(product_id).expect("product not found")
    }
}

mod test;
