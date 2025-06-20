#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};
declare_id!("8B7XpDXjPWodpDUWDSzv4q9k73jB5WdNQXZxNBj1hqw1");

pub const ANCHOR_DISCRIMINATOR: usize = 8;

#[program]
pub mod zero_copy_counter {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        let counter = &mut ctx.accounts.counter.load_init()?;
        counter.count = 0;
        counter.bump = ctx.bumps.counter;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter.load_mut()?;
        counter.count = counter
            .count
            .checked_add(1)
            .ok_or(CustomError::Overflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [b"counter"],
        bump,
        space = ANCHOR_DISCRIMINATOR + Counter::INIT_SPACE,
    )]
    pub counter: AccountLoader<'info, Counter>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"counter"],
        bump = counter.load()?.bump,
    )]
    pub counter: AccountLoader<'info, Counter>,
}

#[account(zero_copy)]

pub struct Counter {
    pub count: u64,
    pub bump: u8,
    pub _padding: [u8; 7], // Align to 16 bytes
}

impl Counter {
    pub const INIT_SPACE: usize = 8 + 1 + 7; // Total = 16 bytes
}

#[error_code]
pub enum CustomError {
    #[msg("Counter overflowed!")]
    Overflow,
}
