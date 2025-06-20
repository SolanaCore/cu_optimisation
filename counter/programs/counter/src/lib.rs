#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("4juyL3gGwJsmKM5pmqyMzz8MDWhMDoofuQ648zSR8oqy");

pub const ANCHOR_DISCRIMINATOR: usize = 8;

#[program]
pub mod counter {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;

        msg!("Counter incremented to {}", counter.count);
        counter.key().log();
        counter.to_account_info().key().log();

        Ok(())
    }
}

// -------- Accounts --------

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [b"counter"],
        bump,
        space = ANCHOR_DISCRIMINATOR + Counter::INIT_SPACE,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,

    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// -------- State --------

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}

impl Counter {
    pub const INIT_SPACE: usize = 8; // Size of u64
}

// -------- Error --------

#[error_code]
pub enum CustomError {
    #[msg("Counter overflowed!")]
    Overflow,
}

