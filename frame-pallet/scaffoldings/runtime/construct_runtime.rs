//! Construct runtime by composing the FRAME pallets into a full runtime.
//!
//! Q. What is a runtime?
//!
//! A. A runtime is the actual implementation of a blockchain. It is the code that is executed
//! on every node in the network. It is the code that validates blocks and executes transactions.
//!
//! Code-wise, it is a struct that has no field for which there are many `Config` trait
//! (defined in each pallet) implementations. The `Config` trait implementations are used to
//! configure the pallets that are used in the runtime.
//!
//! So, a runtime just holds of all the trait implementations for the pallets that are used in
//! the runtime.
//!
//! The runtime has 1 function - `metadata()` which returns the metadata of the runtime.
//!
//! Sources:
//! - https://substrate.stackexchange.com/a/542/2795
//! - https://crates.parity.io/node_template_runtime/struct.Runtime.html

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub struct Runtime
    where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
        Grandpa: pallet_grandpa,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,
        // Include the custom logic from the pallet-template in the runtime.
        TemplateModule: pallet_template,
    }
);
