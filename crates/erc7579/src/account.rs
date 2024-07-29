use crate::types::{PackedUserOperation, Execution, ModeCode, ERC7579Account, SINGLE_EXECUTION_MODE, BATCH_EXECUTION_MODE};
use crate::types::ERC7579Account::ERC7579AccountCalls;
use alloy_primitives::{keccak256, Address, Bytes, B256, U256};
use alloy_sol_types::{abi, sol};
use alloy_sol_types::{sol_data::*, SolValue};


pub struct SmartAccount { 
    pub account_type: AccountType,
    pub is_initialized: bool,
    pub address: Option<Address>,
    pub execution_cache: Option<ERC7579Account::ERC7579AccountCalls>,
}

pub enum AccountType {
    Unknown,
    Safe7579
}


pub trait ExecutionHelper {
    fn encode_execution(&self, executions:Vec<Execution>) -> ERC7579Account::ERC7579AccountCalls;
    fn install_module(&self, module_type: Vec<U256>, module: Address, init_data: Bytes) -> ERC7579Account::ERC7579AccountCalls;
}

impl ExecutionHelper for SmartAccount {
    fn encode_execution(&self, executions:Vec<Execution>) -> ERC7579Account::ERC7579AccountCalls {

        let mode: ModeCode;
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
               mode: mode.into(),
               executionCalldata: Bytes::from(result),
            })
    }

    fn install_module(&self, module_type: Vec<U256>, module: Address, init_data: Bytes) -> ERC7579Account::ERC7579AccountCalls {
        match module_type.len() {
            0 => {
                panic!("No module type to encode")
            },
            1 => {
                ERC7579Account::ERC7579AccountCalls::installModule(
                    ERC7579Account::installModuleCall {
                        moduleTypeId: module_type[0],
                        module,
                        initData: init_data,
                    }
                )
            },
           _ => {
                panic!("Multiple module types not supported")
            }
        }
    }
}

pub trait UserOpBuilder {
    fn get_user_op(&self) -> PackedUserOperation;
}


impl UserOpBuilder for SmartAccount{
    fn get_user_op(&self) -> PackedUserOperation {


        let mut user_op:PackedUserOperation = PackedUserOperation{
            sender: self.address.unwrap(),
            nonce: U256::from(0),
            initCode: Bytes::from(vec![]),
            accountGasLimits: alloy_primitives::FixedBytes([0x00; 32]),
            preVerificationGas: U256::from(0),
            gasFees: alloy_primitives::FixedBytes([0x00; 32]),
            paymasterAndData: Bytes::from(vec![]),
            signature: Bytes::from(vec![]),
            callData: Bytes::from(vec![]),
        };

        user_op

    }
}


