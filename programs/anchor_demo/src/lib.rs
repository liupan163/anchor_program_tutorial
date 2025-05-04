pub mod adapters;
pub mod constants;
pub mod error;
pub mod event;
pub mod instructions;
pub mod macros;
pub mod math;
pub mod states;
pub mod utils;

use anchor_lang::prelude::*;

pub use event::*;

use instructions::*;

declare_id!("2vrz1hEC3HpVxPNXACwy4oMbVqPEw8sA34UYr6KLwQpL");

#[program]
pub mod anchor_demo {

    use super::*;

    // account type & access control case:
    pub fn account_type_demo(ctx: Context<AccountTypeDemo>) -> Result<()> {
        instructions::check_identity_demo(ctx)
    }

    // example method:
    pub fn proxy_swap_demo<'c: 'info, 'info>(
        // Note: lifetime may not live long enough argument requires that ...
        ctx: Context<'_, '_, 'c, 'info, CommissionSOLProxySwapAccounts<'info>>,
        data: SwapArgs,
        commission_rate: u16,
        commission_direction: bool,
        order_id: u64,
    ) -> Result<u64> {
        instructions::proxy_swap_demo(ctx, data, commission_rate, commission_direction, order_id)
    }

    // handle data case: (peekable, slot, )
    pub fn handle_data_demo<'c: 'info, 'info>(
        ctx: Context<'_, '_, 'c, 'info, DataAccounts<'info>>,
    ) -> Result<()> {
        instructions::handle_data(ctx)
    }

    // special token2022 traits case:
    pub fn get_token_traits_demo(ctx: Context<TokenTraitsAccounts>) -> Result<()> {
        instructions::get_token_traits_demo(ctx)
    }

    // multiple signers case:
    pub fn multiple_signers_demo(ctx: Context<MultipleSignersAccounts>) -> Result<()> {
        instructions::multiple_signers_demo(ctx)
    }

    pub fn initialize_signed_msg_user_orders<'c: 'info, 'info>(
        ctx: Context<'_, '_, 'c, 'info, InitializeSignedMsgUserOrders<'info>>,
        num_orders: u16,
    ) -> Result<()> {
        handle_initialize_signed_msg_user_orders(ctx, num_orders)
    }

    // advanced case:
    // reallocate State case:
    pub fn resize_signed_msg_user_orders<'c: 'info, 'info>(
        ctx: Context<'_, '_, 'c, 'info, ResizeSignedMsgUserOrders<'info>>,
        num_orders: u16,
    ) -> Result<()> {
        handle_resize_signed_msg_user_orders(ctx, num_orders)
    }
    // (AccountLoader & AccountInfo) load State case:
    pub fn place_and_make_signed_msg_perp_order<'c: 'info, 'info>(
        ctx: Context<'_, '_, 'c, 'info, PlaceAndMakeSignedMsg<'info>>,
        signed_msg_order_uuid: [u8; 8],
    ) -> Result<()> {
        handle_place_and_make_signed_msg_perp_order(ctx, signed_msg_order_uuid)
    }
}
