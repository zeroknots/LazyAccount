use alloy_primitives::{keccak256, Address, Bytes, B256, U256};
use erc7579::account::*;
use erc7579::types::*;
fn main() {
    println!("Hello, world!");

    let account = SmartAccount {
        account_type: AccountType::Safe7579,
        is_initialized: true,
        address: Address::from([0x42; 20]),
    };

    let execution = account.encode_execution(vec![Execution {
        target: Address::from([0x41; 20]),
        value: U256::from(0),
        callData: Bytes::from([0x41; 20]),
    },
    Execution {
        target: Address::from([0x42; 20]),
        value: U256::from(0),
        callData: Bytes::from([0x42; 20]),
    }

    ]);
    println!("{:?}", execution);
}
