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

#[account]
pub struct Safe {
    pub owner: Pubkey,
    pub name: String,
    pub balance: u64,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_NAME_LENGTH: usize = 40 * 4;
const BALANCE_LENGTH: usize = 8;

impl Safe {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Owner
        + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH // Name
        + BALANCE_LENGTH; // Balance
}