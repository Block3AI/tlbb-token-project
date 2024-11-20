use anchor_lang::prelude::*;

// Replace "YourGeneratedProgramIDHere" with the actual program ID
declare_id!("Euueq2HBsDG4Dwf1QtHRrXCSnmdUQmBXJHqfsWDhYYX1");

#[program]
pub mod tlbb_token {
    use super::*;

    // Add the initialize function here
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let _initializer = &ctx.accounts.payer; // Example usage
        msg!("TLBB Token Initialized Successfully!");
        Ok(())
    }
}

// Define the accounts required for the `initialize` function
#[derive(Accounts)]
pub struct Initialize<'info> {
    // The signer of the transaction
    #[account(mut)]
    pub payer: Signer<'info>,

    // Account to be initialized
    #[account(init, payer = payer, space = 8 + 64)] // Adjust space as needed
    pub my_account: Account<'info, MyData>,

    // System program for account creation
    pub system_program: Program<'info, System>,
}

// Define the account structure for your program
#[account]
pub struct MyData {
    pub value: u64, // Example data field
}