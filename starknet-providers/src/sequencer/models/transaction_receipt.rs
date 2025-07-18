use serde::Deserialize;
use serde_with::serde_as;

use starknet_core::{
    serde::unsigned_field_element::{UfeHex, UfePendingBlockHash},
    types::{ExecutionResources, Felt},
};

use super::{L1Address, TransactionFailureReason};

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ConfirmedReceipt {
    #[serde_as(as = "UfeHex")]
    pub transaction_hash: Felt,
    pub transaction_index: u64,
    // This field is actually always present since v0.12.1, but we're keeping it optional until
    // mainnet is upgraded.
    #[serde(default)]
    pub execution_status: Option<TransactionExecutionStatus>,
    #[serde(default)]
    pub revert_error: Option<String>,
    #[serde(default)]
    pub execution_resources: Option<DetailedExecutionResources>,
    pub l1_to_l2_consumed_message: Option<L1ToL2Message>,
    pub l2_to_l1_messages: Vec<L2ToL1Message>,
    pub events: Vec<Event>,
    #[serde_as(as = "UfeHex")]
    pub actual_fee: Felt,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum TransactionStatus {
    /// Transaction has not been received yet (i.e. not written to storage)
    NotReceived,
    /// Transaction was received by the sequenced
    Received,
    /// Transaction passed the validation and entered the pending block
    Pending,
    /// The transaction failed validation and was skipped (applies both to a
    /// pending and actual created block)
    Rejected,
    Reverted,
    /// Transaction passed the validation and entered a created block
    AcceptedOnL2,
    /// Transaction was accepted on-chain
    AcceptedOnL1,
}

impl TransactionStatus {
    /// Returns `true` if the transaction status is `NotReceived`.
    pub const fn is_not_received(&self) -> bool {
        matches!(self, Self::NotReceived)
    }

    /// Returns `true` if the transaction status is `Received`.
    pub const fn is_received(&self) -> bool {
        matches!(self, Self::Received)
    }

    /// Returns `true` if the transaction status is `Pending`.
    pub const fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns `true` if the transaction status is `Rejected`.
    pub const fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected)
    }

    /// Returns `true` if the transaction status is `Reverted`.
    pub const fn is_reverted(&self) -> bool {
        matches!(self, Self::Reverted)
    }

    /// Returns `true` if the transaction status is `AcceptedOnL2`.
    pub const fn is_accepted_on_l2(&self) -> bool {
        matches!(self, Self::AcceptedOnL2)
    }

    /// Returns `true` if the transaction status is `AcceptedOnL1`.
    pub const fn is_accepted_on_l1(&self) -> bool {
        matches!(self, Self::AcceptedOnL1)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum TransactionExecutionStatus {
    Succeeded,
    Reverted,
    Rejected,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub enum TransactionFinalityStatus {
    NotReceived,
    Received,
    AcceptedOnL2,
    AcceptedOnL1,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct DetailedExecutionResources {
    pub n_steps: u64,
    pub n_memory_holes: u64,
    pub builtin_instance_counter: BuiltinInstanceCounter,
    pub data_availability: Option<ExecutionResources>,
    pub total_gas_consumed: Option<ExecutionResources>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct BuiltinInstanceCounter {
    pub pedersen_builtin: Option<u64>,
    pub range_check_builtin: Option<u64>,
    pub bitwise_builtin: Option<u64>,
    pub output_builtin: Option<u64>,
    pub ecdsa_builtin: Option<u64>,
    pub ec_op_builtin: Option<u64>,
    pub poseidon_builtin: Option<u64>,
    pub keccak_builtin: Option<u64>,
    pub segment_arena_builtin: Option<u64>,
    pub add_mod: Option<u64>,
    pub mul_mod: Option<u64>,
    pub range_check96: Option<u64>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct L1ToL2Message {
    pub from_address: L1Address,
    #[serde_as(as = "UfeHex")]
    pub to_address: Felt,
    #[serde_as(deserialize_as = "UfeHex")]
    pub selector: Felt,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub payload: Vec<Felt>,
    #[serde_as(deserialize_as = "Option<UfeHex>")]
    pub nonce: Option<Felt>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct L2ToL1Message {
    #[serde_as(as = "UfeHex")]
    pub from_address: Felt,
    pub to_address: L1Address,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub payload: Vec<Felt>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct Event {
    #[serde_as(as = "UfeHex")]
    pub from_address: Felt,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub keys: Vec<Felt>,
    #[serde_as(deserialize_as = "Vec<UfeHex>")]
    pub data: Vec<Felt>,
}
