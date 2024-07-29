
use alloy_primitives::{keccak256, Address, Bytes, B256, U256};
use alloy_sol_types::{abi, sol};
use alloy_sol_types::{sol_data::*, SolValue};

use std::fmt;
use std::error::Error;

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
}

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
pub const BATCH_EXECUTION_MODE: ModeCode= ModeCode({
    let mut bytes = [0x00; 32];
        bytes[0] = 0x01;
        alloy_primitives::FixedBytes(bytes)
    });



