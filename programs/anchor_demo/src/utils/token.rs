use crate::constants::{BUMP_SA, SEED_SA};
use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction::transfer};
use anchor_spl::token::Token;
use anchor_spl::token_2022::{self};
use spl_token_2022::extension::transfer_fee::TransferFeeConfig;
use spl_token_2022::extension::{BaseStateWithExtensions, StateWithExtensions};
 
pub fn transfer_sol_from_user<'info>(from: AccountInfo<'info>, to: AccountInfo<'info>, lamports: u64) -> Result<()> {
    if lamports == 0 {
        return Ok(());
    }
    let ix = transfer(from.key, to.key, lamports);
    let res = invoke(&ix, &[from, to]);
    require!(res.is_ok(), ErrorCode::TransferSolFailed);
    Ok(())
}

pub fn transfer_token_from_user<'info>(
    authority: AccountInfo<'info>,
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
    mint_decimals: u8,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    let res = token_2022::transfer_checked(
        CpiContext::new(
            token_program.to_account_info(),
            token_2022::TransferChecked {
                from,
                to,
                authority,
                mint,
            },
        ),
        amount,
        mint_decimals,
    );
    require!(res.is_ok(), ErrorCode::TransferTokenFailed);
    Ok(())
}

pub fn transfer_token_from_sa_pda<'info>(
    authority: AccountInfo<'info>,
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
    mint_decimals: u8,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    let res = token_2022::transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            token_2022::TransferChecked {
                from,
                to,
                authority,
                mint,
            },
            &[&[SEED_SA, &[BUMP_SA]]],
        ),
        amount,
        mint_decimals,
    );
    require!(res.is_ok(), ErrorCode::TransferTokenFailed);
    Ok(())
}


/// Calculate the fee for input amount
pub fn get_transfer_fee(mint_info: &AccountInfo, pre_fee_amount: u64) -> Result<u64> {
    if *mint_info.owner == Token::id() {
        return Ok(0);
    }
    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;

    let fee = if let Ok(transfer_fee_config) = mint.get_extension::<TransferFeeConfig>() {
        transfer_fee_config
            .calculate_epoch_fee(Clock::get()?.epoch, pre_fee_amount)
            .unwrap()
    } else {
        0
    };
    Ok(fee)
}
