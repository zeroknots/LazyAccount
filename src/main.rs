use alloy_contract::SolCallBuilder;
use alloy_network::Ethereum;
use alloy_primitives::{address, keccak256, Address, Bytes, FixedBytes, B256, U256};
use alloy_provider::ProviderBuilder;
use erc7579::account::*;
use erc7579::types::*;
use std::error::Error as StdError;
use std::{fs, str::FromStr};
mod config;
use crate::config::{parse_config, Config};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the JSON input file
    #[arg(short, long)]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let args = Args::parse();
    let config = parse_config(args.config).unwrap();

    run(config).await
}

async fn run(config: Config) -> Result<(), Box<dyn StdError>> {
    // Your async code here
    //
    println!("Hello LazyAccount");

    let mut account = SmartAccount {
        account_type: AccountType::Safe7579,
        is_initialized: true,
        address: Some(Address::from_str(&config.general.account_address)?),
        execution_cache: None,
        validators: None,
    };

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_builtin("http://localhost:8545")
        .await?;

    // account.address = Address::from([0x40; 20]).into();

    let execution = account.encode_execution(vec![
        Execution {
            target: Address::from([0x41; 20]),
            value: U256::from(1),
            callData: Bytes::default()
        },
    ]);

    println!("{:?}", execution);

    let validator_module: Address = address!("903Da2DD182Ea1C962f34282692AA51B81Dc8432");

    let userop = PackedUserOperation::new()
        .with_sender(account.address.expect("UserOp.sender missing"))
        .with_nonce(account.get_nonce(&provider, validator_module).await?)
        .with_signature(Bytes::from([0x40; 20]))
        .with_calldata(execution);
    println!("{:?}", userop);

    account.send_user_op(userop, &provider.clone()).await?;

    Ok(())
}
