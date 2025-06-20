use anchor_lang::prelude::*;

declare_id!("9GcE7VHZfAqPRdy6CTgxdkBwf9359951DLJcDpVbXaAA");

#[program]
pub mod counter_good_approach {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
