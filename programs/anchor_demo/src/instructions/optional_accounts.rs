use crate::error::{ErrorCode, XxxResult};
use crate::states::{Size, User, UserStats};
// use std::cell::RefMut;
use std::convert::TryFrom;

use crate::math::safe_unwrap::SafeUnwrap;
use crate::msg;

use crate::validate;
use anchor_lang::prelude::*;
use anchor_lang::prelude::{AccountLoader, InterfaceAccount};
use anchor_lang::Discriminator;
use anchor_spl::token::TokenAccount;
use anchor_spl::token_interface::{Mint, TokenInterface};
use arrayref::array_ref;
use solana_program::account_info::next_account_info;
use std::iter::Peekable;
// use std::ops::Deref;
use std::slice::Iter;

pub fn deref_case() -> XxxResult {
    // AccountLoader.load() 转换 ->  Ref<>
    // Ref<>.deref()    转换->      Deref()

    Ok(())
}

pub fn load_ref_mut_case(_oracle_account_info: AccountInfo<'_>, _slot: u64) -> XxxResult {
    // let mut oracle: RefMut<PrelaunchOracle> =
    //     load_ref_mut(&oracle_account_info).or(Err(UnableToLoadOracle))?;

    // TODO: NOTE: load_ref_mut(...)  VerSus AccountLoader::try_from(...)

    // oracle.update(xxx, slot)?;

    Ok(())
}

#[allow(clippy::type_complexity)]
pub fn get_referrer_and_referrer_stats<'a>(
    account_info_iter: &mut Peekable<Iter<'a, AccountInfo<'a>>>,
) -> XxxResult<(
    Option<AccountLoader<'a, User>>,
    Option<AccountLoader<'a, UserStats>>,
)> {
    let referrer_account_info = account_info_iter.peek();

    if referrer_account_info.is_none() {
        return Ok((None, None));
    }

    let referrer_account_info = referrer_account_info.safe_unwrap()?;
    let data = referrer_account_info.try_borrow_data().map_err(|e| {
        msg!("{:?}", e);
        ErrorCode::CouldNotDeserializeReferrer
    })?;

    if data.len() < User::SIZE {
        return Ok((None, None));
    }

    let user_discriminator: [u8; 8] = User::discriminator();
    let account_discriminator = array_ref![data, 0, 8];
    if account_discriminator != &user_discriminator {
        return Ok((None, None));
    }

    let referrer_account_info = next_account_info(account_info_iter).safe_unwrap()?;

    validate!(
        referrer_account_info.is_writable,
        ErrorCode::ReferrerMustBeWritable
    )?;

    let referrer: AccountLoader<User> = AccountLoader::try_from(referrer_account_info)
        .or(Err(ErrorCode::CouldNotDeserializeReferrer))?;

    let referrer_stats_account_info = account_info_iter.peek();
    if referrer_stats_account_info.is_none() {
        return Ok((None, None));
    }

    let referrer_stats_account_info = referrer_stats_account_info.safe_unwrap()?;
    let data = referrer_stats_account_info.try_borrow_data().map_err(|e| {
        msg!("{:?}", e);
        ErrorCode::CouldNotDeserializeReferrerStats
    })?;

    if data.len() < UserStats::SIZE {
        return Ok((None, None));
    }

    let user_stats_discriminator: [u8; 8] = UserStats::discriminator();
    let account_discriminator = array_ref![data, 0, 8];
    if account_discriminator != &user_stats_discriminator {
        return Ok((None, None));
    }

    let referrer_stats_account_info = next_account_info(account_info_iter).safe_unwrap()?;

    validate!(
        referrer_stats_account_info.is_writable,
        ErrorCode::ReferrerMustBeWritable
    )?;

    let referrer_stats: AccountLoader<UserStats> =
        AccountLoader::try_from(referrer_stats_account_info)
            .or(Err(ErrorCode::CouldNotDeserializeReferrerStats))?;

    Ok((Some(referrer), Some(referrer_stats)))
}

pub fn get_whitelist_token<'a>(
    account_info_iter: &mut Peekable<Iter<'a, AccountInfo<'a>>>,
) -> XxxResult<Account<'a, TokenAccount>> {
    let token_account_info = account_info_iter.peek();
    if token_account_info.is_none() {
        msg!("Could not find whitelist token");
        return Err(ErrorCode::InvalidWhitelistToken);
    }

    let token_account_info = token_account_info.safe_unwrap()?;
    let whitelist_token: Account<TokenAccount> =
        Account::try_from(token_account_info).map_err(|e| {
            msg!("Unable to deserialize whitelist token");
            msg!("{:?}", e);
            ErrorCode::InvalidWhitelistToken
        })?;

    Ok(whitelist_token)
}

pub fn get_token_interface<'a>(
    account_info_iter: &mut Peekable<Iter<'a, AccountInfo<'a>>>,
) -> XxxResult<Option<Interface<'a, TokenInterface>>> {
    let token_interface_account_info = account_info_iter.peek();
    if token_interface_account_info.is_none() {
        return Ok(None);
    }

    let token_interface_account_info = account_info_iter.next().safe_unwrap()?;
    let token_interface: Interface<TokenInterface> =
        Interface::try_from(token_interface_account_info).map_err(|e| {
            msg!("Unable to deserialize token interface");
            msg!("{:?}", e);
            ErrorCode::DefaultError
        })?;

    Ok(Some(token_interface))
}

pub fn get_token_mint<'a>(
    account_info_iter: &mut Peekable<Iter<'a, AccountInfo<'a>>>,
) -> XxxResult<Option<InterfaceAccount<'a, Mint>>> {
    let mint_account_info = account_info_iter.peek();
    if mint_account_info.is_none() {
        return Ok(None);
    }

    let mint_account_info = account_info_iter.next().safe_unwrap()?;

    match InterfaceAccount::try_from(mint_account_info) {
        Ok(mint) => Ok(Some(mint)),
        Err(_) => Ok(None),
    }
}
