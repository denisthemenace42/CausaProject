use anchor_lang::prelude::*;

declare_id!("5FsRrGxYjW7oV6eUErYzEeAQYR5v6P7m5FU9BzDu8ipk");

#[program]
pub mod project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
