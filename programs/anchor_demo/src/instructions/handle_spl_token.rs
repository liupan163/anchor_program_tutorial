use crate::{
    constants::{SEED_AUTH_STORE, SEED_MINT, SEED_MINT_AUTHORITY},
    states::AuthStore, utils::get_transfer_fee,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use mpl_token_metadata::{instructions::CreateMetadataAccountV3CpiBuilder, types::DataV2};
use std::mem::size_of;

#[derive(Accounts)]
#[instruction(
    params: InitializeParams
)]
pub struct InitializeSplToken<'info> {
    #[account(mut,)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + size_of::<AuthStore>(),
        seeds=[SEED_AUTH_STORE],
        bump,
    )]
    pub auth_store: AccountLoader<'info, AuthStore>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 8,
        mint::authority = mint_authority.key(),
        mint::freeze_authority = admin.key(),
        seeds = [SEED_MINT, params.mint_seed.as_bytes().as_ref()],
        bump,
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK: This is the metadata account for the mint
    #[account(
        mut,
        seeds = [
            "metadata".as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = mpl_token_metadata::ID,
    )]
    pub metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [SEED_MINT_AUTHORITY],
        bump,
    )]
    pub mint_authority: SystemAccount<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    /// CHECK: This is the token metadata program
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitializeParams {
    pub operator_pub_key: Pubkey,
    pub mint_seed: String,
    pub max_supply: u64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub fn initialize_spl_token<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, InitializeSplToken<'info>>,
    params: InitializeParams,
) -> Result<()> {

    // Create metadata for the token
    let seeds = &[
        SEED_MINT_AUTHORITY,
        &[ctx.bumps.mint_authority],
    ];
    let signer = [&seeds[..]];

    let data_v2 = DataV2 {
        name: params.name,
        symbol: params.symbol,
        uri: params.uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    CreateMetadataAccountV3CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .metadata(&ctx.accounts.metadata)
        .mint(&ctx.accounts.mint.to_account_info())
        .mint_authority(&ctx.accounts.mint_authority)
        .payer(&ctx.accounts.admin)
        .update_authority(&ctx.accounts.mint_authority, true)
        .system_program(&ctx.accounts.system_program)
        .data(data_v2)
        .is_mutable(true)
        .invoke_signed(&signer)?;

    Ok(())
}

#[derive(Accounts)]
pub struct SplTokenTraitsAccounts<'info> {
    pub signer_account: Signer<'info>,
    pub mint: Account<'info, Mint>,
}

pub fn get_spl_token_traits<'c: 'info, 'info>(ctx: Context<'_, '_, 'c, 'info, SplTokenTraitsAccounts<'info>>) -> Result<()> {
    let mint_info = &ctx.accounts.mint.to_account_info();
    let transfer_fee = get_transfer_fee(mint_info, 0)?;
    msg!("transfer_fee: {}", transfer_fee);

    Ok(())
}
