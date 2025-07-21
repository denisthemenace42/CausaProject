use anchor_lang::prelude::*;

declare_id!("5FsRrGxYjW7oV6eUErYzEeAQYR5v6P7m5FU9BzDu8ipk");


#[program]
pub mod causa_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Your init logic
        Ok(())
    }

    pub fn claim_reward(ctx: Context<Claim>) -> Result<()> {
        // Your claiming logic
        Ok(())
    }

    pub fn donate_for_rp(ctx: Context<Donate>) -> Result<()> {
        // RP logic here
        Ok(())
    }
}

// Define your context structs
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Vault account state
#[account]
pub struct VaultState {
    pub total_rewards: u64,
}
