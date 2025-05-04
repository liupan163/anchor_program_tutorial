use crate::utils::get_transfer_fee;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

#[derive(Accounts)]
pub struct TokenTraitsAccounts<'info> {
    pub signer_account: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
}

pub fn get_token_traits_demo(ctx: Context<TokenTraitsAccounts>) -> Result<()> {
    let mint_info = &ctx.accounts.mint.to_account_info();
    let transfer_fee = get_transfer_fee(mint_info, 0)?;
    msg!("transfer_fee: {}", transfer_fee);


    Ok(())
}
