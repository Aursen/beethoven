use pinocchio::{ProgramResult, account_info::AccountInfo};

pub trait Quote {
    fn exact_in(amount_in: u64, min_amount_out: u64) -> u64;
    fn exact_out(amount_in: u64, min_amount_out: u64) -> u64;
}

pub trait Deposit {
    fn deposit(amount: u64) -> ProgramResult;
    fn deposit_checked(amount: u64, amount_out: u64) -> ProgramResult;
}

pub trait Withdraw {
    fn withdraw(amount: u64) -> ProgramResult;
    fn withdraw_checked(amount: u64, amount_out: u64) -> ProgramResult;
}

pub trait Repay {
    fn repay(amount: u64) -> ProgramResult;
}

pub trait Borrow<'info> {
    fn borrow(account_infos: &'info [AccountInfo], amount: u64) -> ProgramResult;
}

pub trait Swap {
    fn exact_in(amount_in: u64, min_amount_out: u64) -> ProgramResult;
    fn exact_out(amount_in: u64, min_amount_out: u64) -> ProgramResult;
}