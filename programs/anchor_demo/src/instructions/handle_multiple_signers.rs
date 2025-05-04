use crate::constants::SEED_AUTH_STORE;
use crate::error::ErrorCode;
use crate::states::AuthStore;
use anchor_lang::prelude::*;
use std::collections::HashSet;

#[derive(Accounts)]
#[instruction(market_index: u16,)]
pub struct MultipleSignersAccounts<'info> {
    pub authority1: Signer<'info>,
    pub authority2: Signer<'info>,
    pub authority3: Signer<'info>,

    #[account(
        seeds=[SEED_AUTH_STORE],
        bump
    )]
    pub auth_store: AccountLoader<'info, AuthStore>,
}

pub fn multiple_signers_demo<'info>(ctx: Context<MultipleSignersAccounts<'info>>) -> Result<()> {
    let signer_wallet_list = &ctx.accounts.auth_store.load()?.signer_wallet_list;
    let mut signer_set: HashSet<Pubkey> = HashSet::new();
    for signer in [
        &ctx.accounts.authority1,
        &ctx.accounts.authority2,
        &ctx.accounts.authority3,
    ] {
        signer_set.insert(signer.key());
    }

    let unique_signers: Vec<Pubkey> = signer_set.into_iter().collect();

    let mut signer_count = 0;
    for signer_key in unique_signers {
        if signer_wallet_list.contains(&signer_key) {
            signer_count += 1;
        }
    }
    require!(
        signer_count >= ctx.accounts.auth_store.load()?.signer_threshold,
        ErrorCode::InvalidSignerCount
    );

    // other logic ...

    Ok(())
}
