use alloy_contract::SolCallBuilder;
use alloy_primitives::{address, keccak256, Address, Bytes, B256, U256};
use alloy_provider::Provider;
use alloy_sol_types::{sol_data::*, SolInterface};
use alloy_sol_types::{abi, sol};
use alloy_sol_types::{sol_data::*, SolValue};
use async_trait::async_trait;
use std::error::Error as StdError;

use self::ERC7579Account::ERC7579AccountCalls;
use self::EntryPoint::EntryPointInstance;

sol! {
#[derive(Debug)]
struct PackedUserOperation {
    address sender;
    uint256 nonce;
    bytes initCode;
    bytes callData;
    bytes32 accountGasLimits;
    uint256 preVerificationGas;
    bytes32 gasFees;
    bytes paymasterAndData;
    bytes signature;
}

#[sol(rpc)]
contract EntryPoint {
    function handleOps(
        PackedUserOperation[] calldata ops,
        address payable beneficiary
    ) external;

    function getNonce(address sender, uint192 key) external view returns (uint256 nonce);
}
}

const ENTRYPOINT_ADDR: Address = address!("0000000071727De22E5E9d8BAf0edAc6f37da032");
// fn entry_point<T:Provider>(provider: T) -> EntryPoint::<T> {
//     let ep = EntryPoint::new(ENTRY_POINT, provider)
// }

sol! {

#[derive(Debug, PartialEq, Eq)]
type ModeCode is bytes32;
type CallType is bytes1;
type ExecType is bytes1;
type ModeSelector is bytes4;
type ModePayload is bytes22;

#[derive(Debug, PartialEq, Eq)]
struct Execution {
    address target;
    uint256 value;
    bytes callData;
}




#[derive(Debug, PartialEq, Eq)]
contract ERC7579Account {

    function execute(ModeCode mode, bytes calldata executionCalldata) external ;
    function installModule(
        uint256 moduleTypeId,
        address module,
        bytes calldata initData
    )
        external;

    function uninstallModule(
        uint256 moduleTypeId,
        address module,
        bytes calldata deInitData
    )
        external
        payable;

    function isModuleInstalled(
        uint256 moduleTypeId,
        address module,
        bytes calldata additionalContext
    )
        external
        view
        returns (bool);
}
}

pub const SINGLE_EXECUTION_MODE: ModeCode = ModeCode(alloy_primitives::FixedBytes([0x00; 32]));
pub const BATCH_EXECUTION_MODE: ModeCode = ModeCode({
    let mut bytes = [0x00; 32];
    bytes[0] = 0x01;
    alloy_primitives::FixedBytes(bytes)
});


impl PackedUserOperation {

    pub fn new() -> PackedUserOperation {
        PackedUserOperation {
            sender: Address::default(),
            nonce: U256::from(0),
            initCode: Bytes::default(),
            callData: Bytes::default(),
            accountGasLimits: B256::default(),
            preVerificationGas: U256::from(0),
            gasFees: B256::default(),
            paymasterAndData: Bytes::default(),
            signature: Bytes::default(),
        }
    }
    pub fn with_sender(mut self, sender: Address) -> Self {
        self.sender = sender;
        self
    }

    pub fn with_nonce(mut self, nonce: U256) -> Self {
        self.nonce = nonce;
        self
    }
    pub fn with_init_code(mut self, init_code: Bytes) -> Self {
        self.initCode = init_code;
        self
    }
    pub fn with_calldata(mut self, callData: Bytes) -> Self {
        self.callData = callData;
        self
    }
    pub fn with_account_gas_limits(mut self, account_gas_limits: B256) -> Self {
        self.accountGasLimits = account_gas_limits;
        self
    }
    pub fn with_pre_verification_gas(mut self, pre_verification_gas: U256) -> Self {
        self.preVerificationGas = pre_verification_gas;
        self
    }
    pub fn with_gas_fees(mut self, gas_fees: B256) -> Self {
        self.gasFees = gas_fees;
        self
    }
    pub fn with_paymaster_and_data(mut self, paymaster_and_data: Bytes) -> Self {
        self.paymasterAndData = paymaster_and_data;
        self
    }
    pub fn with_signature(mut self, signature: Bytes) -> Self {
        self.signature = signature;
        self
    }
}
