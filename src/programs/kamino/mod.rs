use pinocchio::{ProgramResult, account_info::AccountInfo, program_error::ProgramError};

use crate::Borrow;

pub struct Kamino;

pub struct BorrowAccounts<'info> {
    pub signer: &'info AccountInfo,
    pub market: &'info AccountInfo,
    pub token_program: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for KaminoBorrowAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [signer, market, token_program] = &accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Check the keys of those accounts
        // Check if they're mutable, etc
        
        Ok(KaminoBorrowAccounts {
            signer,
            market,
            token_program
        })
    }
} 

impl<'info> Borrow<'info> for Kamino {
    fn borrow(account_infos: &'info [AccountInfo], amount: u64) -> ProgramResult {
        // Do some CPI shit here
        let ctx = KaminoBorrowAccounts::try_from(account_infos)?;
        Ok(())
    }
}