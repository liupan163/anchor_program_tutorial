use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::load_mut;
use crate::{
    constants::SIGNED_MSG_PDA_SEED,
    load,
    states::{SignedMsgOrderId, SignedMsgUserOrders, User},
};
use solana_program::rent::Rent;

#[derive(Accounts)]
#[instruction(num_orders: u16)]
pub struct InitializeSignedMsgUserOrders<'info> {
    #[account(
        init,
        seeds = [SIGNED_MSG_PDA_SEED.as_ref(), authority.key().as_ref()],
        space = SignedMsgUserOrders::space(num_orders as usize),
        bump,
        payer = payer
    )]
    pub signed_msg_user_orders: Box<Account<'info, SignedMsgUserOrders>>,
    /// CHECK: Just a normal authority account
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_signed_msg_user_orders<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, InitializeSignedMsgUserOrders<'info>>,
    num_orders: u16,
) -> Result<()> {
    let signed_msg_user_orders = &mut ctx.accounts.signed_msg_user_orders;
    signed_msg_user_orders.authority_pubkey = ctx.accounts.authority.key();
    signed_msg_user_orders
        .signed_msg_order_data
        .resize_with(num_orders as usize, SignedMsgOrderId::default);
    signed_msg_user_orders.validate()?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(num_orders: u16)]
pub struct ResizeSignedMsgUserOrders<'info> {
    #[account(
        mut,
        seeds = [SIGNED_MSG_PDA_SEED.as_ref(), authority.key().as_ref()],
        bump,
        realloc = SignedMsgUserOrders::space(num_orders as usize),
        realloc::payer = payer,
        realloc::zero = false,
    )]
    pub signed_msg_user_orders: Box<Account<'info, SignedMsgUserOrders>>,
    /// CHECK: authority
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_resize_signed_msg_user_orders<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, ResizeSignedMsgUserOrders<'info>>,
    num_orders: u16,
) -> Result<()> {
    let signed_msg_user_orders = &mut ctx.accounts.signed_msg_user_orders;

    signed_msg_user_orders
        .signed_msg_order_data
        .resize_with(num_orders as usize, SignedMsgOrderId::default);
    signed_msg_user_orders.validate()?;
    Ok(())
}

#[derive(Accounts)]
pub struct PlaceAndMakeSignedMsg<'info> {
    #[account(mut)]
    pub user: AccountLoader<'info, User>,

    #[account(
        seeds = [SIGNED_MSG_PDA_SEED.as_ref()],
        bump,
    )]
    /// CHECK: checked in SignedMsgUserOrdersZeroCopy checks
    pub taker_signed_msg_user_orders: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

pub fn handle_place_and_make_signed_msg_perp_order<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, PlaceAndMakeSignedMsg<'info>>,
    signed_msg_order_uuid: [u8; 8],
) -> Result<()> {
    msg!("signed_msg_order_uuid: {:?}", signed_msg_order_uuid);

    // Note: -> Load() implementation.

    // NOTE: keypoint
    // let taker_signed_msg_account = ctx.accounts.taker_signed_msg_user_orders.load()?; 


    // let taker_order_id = taker_signed_msg_account
    //     .iter()
    //     .find(|signed_msg_order_id| signed_msg_order_id.uuid == signed_msg_order_uuid)
    //     .ok_or(ErrorCode::SignedMsgOrderDoesNotExist)?
    //     .order_id;

    let order_exists = load!(ctx.accounts.user)?;
    msg!("order_exists: {:?}", order_exists);
    let mut user = load_mut!(ctx.accounts.user)?;
    msg!("user: {:?}", user);

    Ok(())
}
