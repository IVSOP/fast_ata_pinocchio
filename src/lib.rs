use pinocchio::pubkey::Pubkey;
use pinocchio_pubkey::pubkey;

pub mod instructions;
pub mod utils;

/// For internal use, to get the discriminant of the instruction
#[repr(u8)]
pub(crate) enum Instructions {
    Create = 0,
}

pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
