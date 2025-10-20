//! Module with helper functions like computing an ATA address

use pinocchio::{program_error::ProgramError, pubkey::{create_program_address, try_find_program_address, Pubkey}};

/// Find the ATA address without searching for a bump seed. Returns None in case none was found
pub fn compute_ata(
    owner: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
    associated_token_program: &Pubkey,
) -> Option<(Pubkey, u8)> {
    try_find_program_address(
        &[
            owner,
            token_program,
            mint,
        ],
        associated_token_program,
    )
}

/// Create the ATA address without searching for a bump seed
pub fn compute_ata_bump(
    owner: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
    associated_token_program: &Pubkey,
    bump: u8,
) -> Result<Pubkey, ProgramError> {
    create_program_address(
        &[
            owner,
            token_program,
            mint,
            &[bump],
        ],
        associated_token_program,
    )
}
