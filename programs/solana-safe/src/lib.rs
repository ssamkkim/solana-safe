use anchor_lang::prelude::*;

declare_id!("6vEvbFSmX4gabvWUf5TrAbU3Lw1B2AnKPtRS2kfefnEx");

#[program]
pub mod solana_safe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
