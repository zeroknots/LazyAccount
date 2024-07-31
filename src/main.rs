use alloy_contract::SolCallBuilder;
use alloy_network::{EthereumWallet};
use alloy_primitives::{address, keccak256, Address, Bytes, FixedBytes, B256, U256};
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
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
    #[arg(short, long)]
    private_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let args = Args::parse();
    let config = parse_config(args.config).unwrap();

    run(config, args.private_key).await
}

async fn run(config: Config, priv_key:String) -> Result<(), Box<dyn StdError>> {
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


    let signer = PrivateKeySigner::from_str(&priv_key)?;
    let wallet = EthereumWallet::from(signer);


    let rpc_url = "http://localhost:8545";
    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_http(rpc_url.parse()?);

    println!("{:?}", provider);

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

    let mut key_bytes = [0u8; 32];
    key_bytes[12..32].copy_from_slice(&validator_module.as_slice());
    let key = U256::from_be_bytes(key_bytes);
    // Truncate to 192 bits (24 bytes)
    let key = key & (U256::MAX >> 64); // Equivalent to uint192 in Solidity
    let ep: Address = address!("0000000071727De22E5E9d8BAf0edAc6f37da032");
    let contract = EntryPoint::new(ep, provider);
    let EntryPoint::getNonceReturn { nonce } = contract
        .getNonce(account.address.ok_or("No address")?, key)
        .call()
        .await?;
    println!("Nonce: {:?}", nonce);

    let userop = PackedUserOperation::new()
        .with_sender(account.address.expect("UserOp.sender missing"))
        .with_nonce(nonce)
        .with_signature(Bytes::from([0x40; 20]))
        .with_calldata(execution);
    println!("{:?}", userop);

    // account.send_user_op(userop, &provider.clone()).await?;

    Ok(())
}
