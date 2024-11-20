use anchor_lang::prelude::*;

declare_id!("YourProgramIDHere");

#[program]
pub mod tlbb_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("TLBB Token Initialized");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {}