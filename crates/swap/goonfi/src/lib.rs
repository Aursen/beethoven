#![no_std]

use {
    beethoven_core::Swap,
    core::mem::MaybeUninit,
    solana_account_view::AccountView,
    solana_address::Address,
    solana_instruction_view::{
        cpi::{invoke_signed, Signer},
        InstructionAccount, InstructionView,
    },
    solana_program_error::{ProgramError, ProgramResult},
};

pub const GOONFI_PROGRAM_ID: Address =
    Address::from_str_const("goonERTdGsjnkZqWuVjs73BZ3Pb9qoCUdBUL17BnS5j");

const SWAP_DISCRIMINATOR: u8 = 2;

pub struct Goonfi;

pub struct GoonfiSwapData {
    pub is_bid: bool,
    pub is_ultra: bool,
    pub bump: u8,
}

impl TryFrom<&[u8]> for GoonfiSwapData {
    type Error = ProgramError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 20 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self {
            is_bid: data[1] != 0,
            is_ultra: data[19] != 0,
            bump: data[2],
        })
    }
}

pub struct GoonfiSwapAccounts<'info> {
    pub goonfi_program: &'info AccountView,
    pub payer: &'info AccountView,
    pub market: &'info AccountView,
    pub base_token_account: &'info AccountView,
    pub quote_token_account: &'info AccountView,
    pub base_vault: &'info AccountView,
    pub quote_vault: &'info AccountView,
    pub blacklist: &'info AccountView,
    pub instructions_sysvar: &'info AccountView,
    pub token_program: &'info AccountView,
}

pub const GOONFI_SWAP_IX_ACCOUNTS_LEN: usize = 9;

impl<'info> TryFrom<&'info [AccountView]> for GoonfiSwapAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountView]) -> Result<Self, Self::Error> {
        if accounts.len() < 10 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        let [goonfi_program, payer, market, base_token_account, quote_token_account, base_vault, quote_vault, blacklist, instructions_sysvar, token_program, ..] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        Ok(Self {
            goonfi_program,
            payer,
            market,
            base_token_account,
            quote_token_account,
            base_vault,
            quote_vault,
            blacklist,
            instructions_sysvar,
            token_program,
        })
    }
}

impl<'info> Swap<'info> for Goonfi {
    type Accounts = GoonfiSwapAccounts<'info>;
    type Data = GoonfiSwapData;

    fn swap_signed(
        ctx: &Self::Accounts,
        in_amount: u64,
        minimum_out_amount: u64,
        data: &Self::Data,
        signer_seeds: &[Signer],
    ) -> ProgramResult {
        let accounts = [
            InstructionAccount::readonly_signer(ctx.payer.address()),
            InstructionAccount::writable(ctx.market.address()),
            InstructionAccount::writable(ctx.base_token_account.address()),
            InstructionAccount::writable(ctx.quote_token_account.address()),
            InstructionAccount::writable(ctx.base_vault.address()),
            InstructionAccount::writable(ctx.quote_vault.address()),
            InstructionAccount::readonly(ctx.blacklist.address()),
            InstructionAccount::readonly(ctx.instructions_sysvar.address()),
            InstructionAccount::readonly(ctx.token_program.address()),
        ];

        let account_infos = [
            ctx.payer,
            ctx.market,
            ctx.base_token_account,
            ctx.quote_token_account,
            ctx.base_vault,
            ctx.quote_vault,
            ctx.blacklist,
            ctx.instructions_sysvar,
            ctx.token_program,
        ];

        let mut instruction_data = MaybeUninit::<[u8; 20]>::uninit();
        unsafe {
            let ptr = instruction_data.as_mut_ptr() as *mut u8;
            core::ptr::write(ptr, SWAP_DISCRIMINATOR);
            core::ptr::write(ptr.add(1), data.is_bid as u8);
            core::ptr::write(ptr.add(2), data.bump);
            core::ptr::copy_nonoverlapping(in_amount.to_le_bytes().as_ptr(), ptr.add(3), 8);
            core::ptr::copy_nonoverlapping(
                minimum_out_amount.to_le_bytes().as_ptr(),
                ptr.add(11),
                8,
            );
            core::ptr::write(ptr.add(19), data.is_ultra as u8);
        }

        let instruction = InstructionView {
            program_id: &GOONFI_PROGRAM_ID,
            accounts: &accounts,
            data: unsafe { instruction_data.assume_init_ref() },
        };

        invoke_signed(&instruction, &account_infos, signer_seeds)
    }

    fn swap(
        ctx: &Self::Accounts,
        in_amount: u64,
        minimum_out_amount: u64,
        data: &Self::Data,
    ) -> ProgramResult {
        Self::swap_signed(ctx, in_amount, minimum_out_amount, data, &[])
    }
}
