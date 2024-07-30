use crate::types::ERC7579Account::ERC7579AccountCalls;
use crate::types::{
    ERC7579Account, Execution, ModeCode, PackedUserOperation, BATCH_EXECUTION_MODE, EntryPoint,
    SINGLE_EXECUTION_MODE,
};
use alloy_primitives::{keccak256, address, Address, Bytes, B256, U256};
use alloy_provider::Provider;
use alloy_sol_types::{abi, sol};
use alloy_sol_types::{sol_data::*, SolValue, SolInterface, SolCall};
use async_trait::async_trait;
use serde::{Deserialize, Deserializer};
use std::error::Error as StdError;

#[derive(Debug,  PartialEq, Eq)]
pub struct SmartAccount {
    pub account_type: AccountType,
    pub is_initialized: bool,
    pub address: Option<Address>,
    pub execution_cache: Option<ERC7579Account::ERC7579AccountCalls>,
    pub validators: Option<Vec<Address>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum AccountType {
    Unknown,
    Safe7579,
    Kernel,
}




pub trait ExecutionHelper {
    fn encode_execution(&self, executions: Vec<Execution>) -> Bytes;
    fn install_module(
        &self,
        module_type: Vec<U256>,
        module: Address,
        init_data: Bytes,
    ) -> ERC7579Account::ERC7579AccountCalls;
}

impl ExecutionHelper for SmartAccount {
    fn encode_execution(&self, executions: Vec<Execution>) -> Bytes{
        let mode: ModeCode;
        let mut result: Vec<u8> = Vec::new();

        match executions.len() {
            0 => {
                panic!("No executions to encode")
            }
            1 => {
                let tmp = Execution::abi_encode_packed(&executions[0]);
                result.extend(tmp);
                mode = SINGLE_EXECUTION_MODE;
            }
            _ => {
                mode = BATCH_EXECUTION_MODE;
                for execution in executions {
                    let tmp = Execution::abi_encode(&execution);
                    result.extend(tmp);
                }
            }
        }

        let calldata = ERC7579Account::executeCall {
            mode: mode.into(),
            executionCalldata: result.into(),
        };
        Bytes::from(calldata.abi_encode())
    }

    fn install_module(
        &self,
        module_type: Vec<U256>,
        module: Address,
        init_data: Bytes,
    ) -> ERC7579Account::ERC7579AccountCalls {
        match module_type.len() {
            0 => {
                panic!("No module type to encode")
            }
            1 => ERC7579Account::ERC7579AccountCalls::installModule(
                ERC7579Account::installModuleCall {
                    moduleTypeId: module_type[0],
                    module,
                    initData: init_data,
                },
            ),
            _ => {
                panic!("Multiple module types not supported")
            }
        }
    }
}

#[async_trait]
pub trait BaseAccount {
    async fn get_nonce<T: Provider + Send + Sync>(
        &self,
        provider: &T,
        validator_module: Address,
    ) -> Result<U256, Box<dyn StdError>>;
}

#[async_trait]
impl BaseAccount for SmartAccount {
    async fn get_nonce<T: Provider + Send + Sync>(
        &self,
        provider: &T,
        validator_module: Address,
    ) -> Result<U256, Box<dyn StdError>> {
        let mut key_bytes = [0u8; 32];
        key_bytes[12..32].copy_from_slice(&validator_module.as_slice());
        let key = U256::from_be_bytes(key_bytes);
        // Truncate to 192 bits (24 bytes)
        let key = key & (U256::MAX >> 64); // Equivalent to uint192 in Solidity
        let ep: Address = address!("0000000071727De22E5E9d8BAf0edAc6f37da032");
        let contract = EntryPoint::new(ep, provider);
        let EntryPoint::getNonceReturn { nonce } = contract
            .getNonce(self.address.ok_or("No address")?, key)
            .call()
            .await?;
        println!("Nonce: {:?}", nonce);
        Ok(nonce)
    }
}


#[async_trait]
pub trait Bundler {
    async fn send_user_op<T: Provider>(&self,userop:PackedUserOperation, provider:&T) -> Result<(), Box<dyn StdError>>;
}

#[async_trait]
impl Bundler for SmartAccount {
    async fn send_user_op<T: Provider> (&self,userop:PackedUserOperation, provider:&T) -> Result<(), Box<dyn StdError>> { 
        let ep: Address = address!("0000000071727De22E5E9d8BAf0edAc6f37da032");
        let contract = EntryPoint::new(ep, provider);
        let EntryPoint::handleOpsReturn {} = contract.handleOps(vec![userop], ep).call().await?;
        Ok(())

    }
}
