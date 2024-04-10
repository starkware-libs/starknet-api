use crate::core::ContractAddress;
use crate::state::ContractClass;
use crate::transaction::{
    DeclareTransaction, DeployAccountTransaction, InvokeTransaction, TransactionHash,
};

/// Represents a paid Starknet transaction.
#[derive(Clone, Debug)]
pub enum InternalTransaction {
    Declare(InternalDeclareTransaction),
    DeployAccount(InternalDeployAccountTransaction),
    Invoke(InternalInvokeTransaction),
}

// TODO(Mohammad): Add constructor for all the transaction's structs.
#[derive(Clone, Debug)]
pub struct InternalDeclareTransaction {
    pub tx: DeclareTransaction,
    pub tx_hash: TransactionHash,
    // Indicates the presence of the only_query bit in the version.
    pub only_query: bool,
    pub class_info: ClassInfo,
}

#[derive(Clone, Debug)]
pub struct InternalDeployAccountTransaction {
    pub tx: DeployAccountTransaction,
    pub tx_hash: TransactionHash,
    pub contract_address: ContractAddress,
    // Indicates the presence of the only_query bit in the version.
    pub only_query: bool,
}

#[derive(Clone, Debug)]
pub struct InternalInvokeTransaction {
    pub tx: InvokeTransaction,
    pub tx_hash: TransactionHash,
    // Indicates the presence of the only_query bit in the version.
    pub only_query: bool,
}

#[derive(Clone, Debug)]
pub struct ClassInfo {
    pub contract_class: ContractClass,
    pub sierra_program_length: usize,
    pub abi_length: usize,
}
