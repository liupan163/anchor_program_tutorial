use crate::{
    constants::{MOCK_OWNER, SEED_AUTH_STORE, X_AUTHORITY},
    error::ErrorCode,
    states::{AuthStore, SignedMsgWsDelegates},
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use std::{cell::RefMut, ops::DerefMut, panic::Location};

#[derive(Accounts)]
pub struct AccountTypeDemo<'info> {
    #[account(mut)]
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

    #[account(
        associated_token::mint = source_mint,
        associated_token::authority = meme_authority,
        associated_token::token_program = transfer_token_program,
    )]
    pub source_token_sa: Option<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        seeds = [b"signed_msg_ws_delegates", signer1.key().as_ref()],
        bump,
        init,
        space = 8 + 4 + 100 * 32,  // NOTE: fixed size account.
        payer=signer1
    )]
    pub signed_msg_ws_delegates: Account<'info, SignedMsgWsDelegates>,

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
    pub system_program: Program<'info, System>,
}

#[access_control(check_owner(&ctx))]
pub fn account_type_demo<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, AccountTypeDemo<'info>>,
) -> Result<()> {
    let signed_msg_ws_delegates: &mut SignedMsgWsDelegates =
        ctx.accounts.signed_msg_ws_delegates.deref_mut();
    msg!("signed_msg_ws_delegates: {:?}", signed_msg_ws_delegates);

    // AccountLoader::try_from()
    let account_info = ctx.remaining_accounts.get(0).unwrap();
    let account_loader: AccountLoader<AuthStore> =
        AccountLoader::try_from(account_info).or(Err(ErrorCode::InvalidAuthStore))?;
    let auth_store: RefMut<AuthStore> = account_loader.load_mut()?;    
    msg!("auth_store: {:?}", auth_store);


    // consume remaining Peekable Account.
    // NOTE: peekable DATA， can see it and not consume it.
    let remaining_accounts_iter = &mut ctx.remaining_accounts.iter().peekable();
    
    let tobe_peek_account = remaining_accounts_iter.peek();
    match tobe_peek_account {
        Some(account) => {
            // look data beforehand
            let account_info = account.to_account_info();
            msg!("peek account: {:?}", account_info.key());

            // Note: consume the peeked account
            remaining_accounts_iter.next();
        }
        None => {
            let caller = Location::caller();
            msg!(
                "Could not find account at {}:{}",
                caller.file(),
                caller.line()
            );
        }
    }

    Ok(())
}

pub fn check_owner(ctx: &Context<AccountTypeDemo>) -> Result<()> {
    // Check if signer === owner
    require_keys_eq!(
        ctx.accounts.signer1.key(),
        MOCK_OWNER.parse::<Pubkey>().unwrap(),
        ErrorCode::NotOwner
    );
    Ok(())
}
