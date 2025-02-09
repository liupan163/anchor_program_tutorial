use crate::error::ErrorCode;
use crate::MOCK_OWNER;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    pub signer_account: Signer<'info>,
}

#[access_control(check_owner(ctx))]
pub fn check_identity(ctx: Context<OnlyOwner>) -> Result<()> {
    Ok(())
}

pub fn check_owner(ctx: Context<OnlyOwner>) -> Result<()> {
    // Check if signer === owner
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        MOCK_OWNER.parse::<Pubkey>().unwrap(),
        ErrorCode::NotOwner
    );
    Ok(())
}
