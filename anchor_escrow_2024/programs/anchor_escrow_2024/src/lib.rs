use anchor_lang::prelude::*;

mod context;
use context::*;
mod state;

declare_id!("B5faTwrWWZA1nQqGZ2aGjXYGyjYRHLFJ8LNzhHhwDRD7");

#[program]
pub mod anchor_escrow_2024 {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, receive: u64) -> Result<()> {
        ctx.accounts.save_escrow(seed, receive, ctx.bumps.escrow)?;
        // ctx.accounts.deposit_to_vault(amount)
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer_to_maker()?;
        ctx.accounts.withdraw_and_close()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close()
    }
}
