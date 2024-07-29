use crate::types::{Execution, ModeCode, ERC7579Account, SINGLE_EXECUTION_MODE, BATCH_EXECUTION_MODE};
use crate::types::ERC7579Account::ERC7579AccountCalls;
use alloy_primitives::{keccak256, Address, Bytes, B256, U256};
use alloy_sol_types::{abi, sol};
use alloy_sol_types::{sol_data::*, SolValue};


pub struct SmartAccount { 
    pub account_type: AccountType,
    pub is_initialized: bool,
    pub address: Address,
}

pub enum AccountType {
    Unknown,
    Safe7579
}


pub trait ExecutionHelper {
    fn encode_execution(&self, executions:Vec<Execution>) -> ERC7579Account::ERC7579AccountCalls;
}

impl ExecutionHelper for SmartAccount {
    fn encode_execution(&self, executions:Vec<Execution>) -> ERC7579Account::ERC7579AccountCalls {

        let mode: alloy_primitives::FixedBytes<32>;
        let mut result:Vec<u8> = Vec::new();

        match executions.len() {
            0 => {
                panic!("No executions to encode")
            },
            1 => {
                let tmp = Execution::abi_encode_packed(&executions[0]);
                result.extend(tmp);
                mode = SINGLE_EXECUTION_MODE;
            },
            _ => {
                mode = BATCH_EXECUTION_MODE;
                for execution in executions {
                    let  tmp = Execution::abi_encode(&execution);
                    result.extend(tmp);
                }
            }
        }

      ERC7579Account::ERC7579AccountCalls::execute(
            ERC7579Account::executeCall {
               mode,
               executionCalldata: Bytes::from(result),
            })
    }
}


