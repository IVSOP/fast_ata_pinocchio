use pinocchio::{account_info::AccountInfo, cpi::invoke_signed, instruction::{AccountMeta, Instruction, Signer}, ProgramResult};

/// Creates an ATA account by calling the Create instruction from the Associated Token Program.
/// ### Accounts:
///   0. `[WRITE, SIGNER]` Funding account (must be a system account)
///   1. `[WRITE]` Associated token account address to be created
///   2. `[]` Wallet address for the new associated token account
///   3. `[]` The token mint for the new associated token account
///   4. `[]` System program
///   5. `[]` SPL Token program
///   6. `[]` SPL Associated token program
pub fn create_ata(
    funding_account: &AccountInfo,
    signers: &[Signer],
    ata: &AccountInfo,
    owner: &AccountInfo,
    mint: &AccountInfo,
    system_program: &AccountInfo,
    token_program: &AccountInfo,
    associated_token_program: &AccountInfo,
) -> ProgramResult {
    let account_metas: [AccountMeta; 6] = [
        AccountMeta::writable_signer(funding_account.key()),
        AccountMeta::writable(ata.key()),
        AccountMeta::readonly(owner.key()),
        AccountMeta::readonly(mint.key()),
        AccountMeta::readonly(system_program.key()),
        AccountMeta::readonly(token_program.key()),
    ];

    let instruction = Instruction {
        program_id: associated_token_program.key(),
        accounts: &account_metas,
        data: &[crate::Instructions::Create as u8],
    };

    let account_infos = &[
        funding_account,
        ata,
        owner,
        mint,
        system_program,
        token_program,
    ];

    invoke_signed(&instruction, account_infos, signers)
}
