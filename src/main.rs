use std::{sync::Arc, str::FromStr};

use ethers::{prelude::{Provider, Http, abigen}, types::H160};
use adapters::test_abi::test_abi::test_abi;

#[tokio::main]
async fn main() {
    let client = Arc::new(Provider::<Http>::try_from("testurl").unwrap());
    let test = test_abi::new(H160::from_str("").unwrap(), client);
}
