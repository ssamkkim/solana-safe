use anchor_lang::prelude::*;

declare_id!("6vEvbFSmX4gabvWUf5TrAbU3Lw1B2AnKPtRS2kfefnEx");

#[program]
pub mod solana_safe {
    use super::*;

    pub fn initialize_safe(ctx: Context<InitializeSafe>, name: String) -> Result<()> {
        let safe: &mut Account<Safe> = &mut ctx.accounts.safe;
        let owner: &Signer = &ctx.accounts.owner;

        if name.chars().count() > 40 {
            return Err(error!(ErrorCode::NameTooLong))
        }

        safe.owner = *owner.key;
        safe.name = name;
        safe.balance = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeSafe<'info> {
    #[account(init, payer = owner, space = Safe::LEN)]
    pub safe: Account<'info, Safe>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

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

#[error_code]
pub enum ErrorCode  {
    #[msg("The provided name should be 40 characters long maximum.")]
    NameTooLong,
}