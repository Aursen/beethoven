use pinocchio::{account_info::AccountInfo, instruction::Signer, ProgramResult};

pub trait Deposit<'info> {
    type Accounts;

    fn deposit_signed(ctx: &Self::Accounts, amount: u64, signer_seeds: &[Signer]) -> ProgramResult;
    fn deposit(ctx: &Self::Accounts, amount: u64) -> ProgramResult;
}

pub trait Withdraw<'info> {
    fn withdraw(account_infos: &'info [AccountInfo], amount: u64) -> ProgramResult;
}
