use crate::{
    constants::{MOCK_OWNER, SEED_AUTH_STORE, X_AUTHORITY},
    error::ErrorCode,
    states::AuthStore,
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct AccountTypeDemo<'info> {
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,

    #[account(
        mut,
        constraint = admin.key() == Pubkey::default() @ErrorCode::InvalidAdmin,
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub operator: Signer<'info>,

    #[account(
        mut,
        seeds=[X_AUTHORITY],
        bump
    )]
    pub meme_authority: SystemAccount<'info>,

    #[account(
        mut,
        seeds=[SEED_AUTH_STORE],
        bump
    )]
    pub auth_store: AccountLoader<'info, AuthStore>,

    #[account(
        mut,
        token::mint = source_mint,
    )]
    pub source_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub middle_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = destination_mint,
    )]
    pub destination_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub source_mint: InterfaceAccount<'info, Mint>,
    pub destination_mint: InterfaceAccount<'info, Mint>,

    pub transfer_token_program: Interface<'info, TokenInterface>,
}

#[access_control(check_owner(ctx))]
pub fn check_identity_demo(ctx: Context<AccountTypeDemo>) -> Result<()> {
    Ok(())
}

pub fn check_owner(ctx: Context<AccountTypeDemo>) -> Result<()> {
    // Check if signer === owner
    require_keys_eq!(
        ctx.accounts.signer1.key(),
        MOCK_OWNER.parse::<Pubkey>().unwrap(),
        ErrorCode::NotOwner
    );
    Ok(())
}
