use crate::constants::authority_pda;
use crate::states::{User, UserStats};

use crate::error::ErrorCode;
use crate::validate;
use crate::load_mut;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

use super::get_token_mint;

#[derive(Accounts)]
#[instruction(market_index: u16,)]
pub struct DataAccounts<'info> {
    #[account(
        mut,
        constraint = user.key() == authority_pda::id() @ErrorCode::InvalidAdmin,
    )]
    pub user: AccountLoader<'info, User>,
    #[account(mut)]
    pub user_stats: AccountLoader<'info, UserStats>,
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"spot_market_vault".as_ref(), market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub spot_market_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = &spot_market_vault.mint.eq(&user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handle_data<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, DataAccounts<'info>>,
) -> Result<()> {
    let user = &mut load_mut!(ctx.accounts.user)?;

    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    let slot = clock.slot;
    msg!("now: {}", now);
    msg!("slot: {}", slot);

    let remaining_accounts_iter = &mut ctx.remaining_accounts.iter().peekable();
    // NOTE: peekable DATA， can see it and not consume it.
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
            msg!("no account to peek");
        }
    }

    let mint = get_token_mint(remaining_accounts_iter)?;
    msg!("mint: {:? }", mint);
    validate!(!user.has_open_order, ErrorCode::TooManyHops)?;

    Ok(())
}
