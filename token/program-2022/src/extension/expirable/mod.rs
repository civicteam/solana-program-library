use {
    crate::{
        error::TokenError,
        extension::{Extension, ExtensionType},
        pod::*,
    },
    bytemuck::{Pod, Zeroable},
    solana_program::{clock::Epoch, entrypoint::ProgramResult},
    std::{cmp, convert::TryFrom},
};

/// Transfer fee extension instructions
pub mod instruction;

/// Transfer fee extension processor
pub mod processor;

/// Transfer fee information
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct Expiry {
    /// unix timestamp when the token expires
    pub expiry_timestamp: PodU64, // Epoch,
}
impl Extension for Expiry {
    const TYPE: ExtensionType = ExtensionType::Expiry;
}