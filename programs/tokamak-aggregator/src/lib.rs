use anchor_lang::prelude::*;

declare_id!("FQof46Y4b7ff7NnkdAAydmXnY6KvcZMwdJkLLwGStMpU");

#[program]
pub mod tokamak_aggregator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
