use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

/// Create an ATA account by calling the Create instruction from the Associated Token Program.
/// ### Accounts:
///   0. `[WRITE, SIGNER]` Funding account (must be a system account)
///   1. `[WRITE]` Associated token account address to be created
///   2. `[]` Wallet address for the new associated token account
///   3. `[]` The token mint for the new associated token account
///   4. `[]` System program
///   5. `[]` SPL Token program
///   6. `[]` SPL Associated token program
pub struct CreateAta<'a> {
    pub funding_account: &'a AccountInfo,
    pub ata: &'a AccountInfo,
    pub owner: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub associated_token_program: &'a AccountInfo,
}

impl CreateAta<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let account_metas: [AccountMeta; 6] = [
            AccountMeta::writable_signer(self.funding_account.key()),
            AccountMeta::writable(self.ata.key()),
            AccountMeta::readonly(self.owner.key()),
            AccountMeta::readonly(self.mint.key()),
            AccountMeta::readonly(self.system_program.key()),
            AccountMeta::readonly(self.token_program.key()),
        ];

        let instruction = Instruction {
            program_id: self.associated_token_program.key(),
            accounts: &account_metas,
            data: &[crate::Instructions::Create as u8],
        };

        let account_infos = &[
            self.funding_account,
            self.ata,
            self.owner,
            self.mint,
            self.system_program,
            self.token_program,
        ];

        invoke_signed(&instruction, account_infos, signers)
    }
}
