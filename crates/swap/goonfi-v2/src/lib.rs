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

pub const GOONFI_V2_PROGRAM_ID: Address =
    Address::from_str_const("goonuddtQRrWqqn5nFyczVKaie28f3kDkHWkHtURSLE");

const SWAP_DISCRIMINATOR: u8 = 1;

pub struct GoonfiV2;

pub struct GoonfiV2SwapData {
    pub is_bid: bool,
    pub is_ultra: bool,
}

impl TryFrom<&[u8]> for GoonfiV2SwapData {
    type Error = ProgramError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 2 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self {
            is_bid: data[0] != 0,
            is_ultra: data[1] != 0,
        })
    }
}

pub struct GoonfiV2SwapAccounts<'info> {
    pub goonfi_v2_program: &'info AccountView,
    pub payer: &'info AccountView,
    pub market: &'info AccountView,
    pub base_token_account: &'info AccountView,
    pub quote_token_account: &'info AccountView,
    pub base_vault: &'info AccountView,
    pub quote_vault: &'info AccountView,
    pub base_mint: &'info AccountView,
    pub quote_mint: &'info AccountView,
    pub oracle: &'info AccountView,
    pub blacklist: &'info AccountView,
    pub instructions_sysvar: &'info AccountView,
    pub base_token_program: &'info AccountView,
    pub quote_token_program: &'info AccountView,
}

impl<'info> TryFrom<&'info [AccountView]> for GoonfiV2SwapAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountView]) -> Result<Self, Self::Error> {
        if accounts.len() < 14 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        let [goonfi_v2_program, payer, market, base_token_account, quote_token_account, base_vault, quote_vault, base_mint, quote_mint, oracle, blacklist, instructions_sysvar, base_token_program, quote_token_program, ..] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        Ok(Self {
            goonfi_v2_program,
            payer,
            market,
            base_token_account,
            quote_token_account,
            base_vault,
            quote_vault,
            base_mint,
            quote_mint,
            oracle,
            blacklist,
            instructions_sysvar,
            base_token_program,
            quote_token_program,
        })
    }
}

impl<'info> Swap<'info> for GoonfiV2 {
    type Accounts = GoonfiV2SwapAccounts<'info>;
    type Data = GoonfiV2SwapData;

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
            InstructionAccount::readonly(ctx.base_mint.address()),
            InstructionAccount::readonly(ctx.quote_mint.address()),
            InstructionAccount::readonly(ctx.oracle.address()),
            InstructionAccount::readonly(ctx.blacklist.address()),
            InstructionAccount::readonly(ctx.instructions_sysvar.address()),
            InstructionAccount::readonly(ctx.base_token_program.address()),
            InstructionAccount::readonly(ctx.quote_token_program.address()),
        ];

        let account_infos = [
            ctx.payer,
            ctx.market,
            ctx.base_token_account,
            ctx.quote_token_account,
            ctx.base_vault,
            ctx.quote_vault,
            ctx.base_mint,
            ctx.quote_mint,
            ctx.oracle,
            ctx.blacklist,
            ctx.instructions_sysvar,
            ctx.base_token_program,
            ctx.quote_token_program,
        ];

        let mut instruction_data = MaybeUninit::<[u8; 19]>::uninit();
        unsafe {
            let ptr = instruction_data.as_mut_ptr() as *mut u8;
            core::ptr::write(ptr, SWAP_DISCRIMINATOR);
            core::ptr::write(ptr.add(1), data.is_bid as u8);
            core::ptr::copy_nonoverlapping(in_amount.to_le_bytes().as_ptr(), ptr.add(2), 8);
            core::ptr::copy_nonoverlapping(
                minimum_out_amount.to_le_bytes().as_ptr(),
                ptr.add(10),
                8,
            );
            core::ptr::write(ptr.add(18), data.is_ultra as u8);
        }

        let instruction = InstructionView {
            program_id: &GOONFI_V2_PROGRAM_ID,
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
