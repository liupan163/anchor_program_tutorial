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

    // --------- Start: SPL related case ---------

    // initialize mint
    pub fn initialize_spl_mint<'c: 'info, 'info>(ctx: Context<'_, '_, 'c, 'info, InitializeSplToken<'info>>, params: InitializeParams) -> Result<()> {
        instructions::initialize_spl_token(ctx, params)
    }

    // special token2022 traits case:
    pub fn get_spl_token_traits<'c: 'info, 'info>(ctx: Context<'_, '_, 'c, 'info, SplTokenTraitsAccounts<'info>>) -> Result<()> {
        instructions::get_spl_token_traits(ctx)
    }
    // --------- End: SPL related case ---------



    // ------------ Start: Account and DataSize case ------------

    //  1. Common AccountType.
    //  2. Create Fixed size Array AccountType.
    //  3. Consume remainingAccount Peekable Type.
    //  4. Access control case.
    //  5. Convert RemainingAccounts[0] to---> Specific AccountType.
    pub fn account_type_demo<'c: 'info, 'info>(ctx: Context<'_, '_, 'c, 'info, AccountTypeDemo<'info>>) -> Result<()> {
        instructions::account_type_demo(ctx)
    }

    // Create Account with Customized-Size Array AccountType.
    // scenario 1: different size data for different rent cost,
    pub fn initialize_custom_size_data<'c: 'info, 'info>(
        ctx: Context<'_, '_, 'c, 'info, InitializeSignedMsgUserOrders<'info>>,
        num_orders: u16,
    ) -> Result<()> {
        handle_initialize_custom_size_data(ctx, num_orders)
    }

    // reallocate Account Size case:
    // scenario 1: resize it with different rent cost,
    pub fn resize_custom_size_data<'c: 'info, 'info>(
        ctx: Context<'_, '_, 'c, 'info, ResizeSignedMsgUserOrders<'info>>,
        num_orders: u16,
    ) -> Result<()> {
        handle_resize_custom_size_data(ctx, num_orders)
    }
    // --------- End: Account and DataSize case ---------



    // --------- Start: (AccountLoader & AccountInfo) load State case ---------

    // scenario 1: impl `CustomizedTrait` for AccountInfo,
    // scenario 2:      use `bytemuck` to convert data to structured data.
    // scenario 3: `AccountLoader` type. to load data.
    pub fn place_and_make_signed_msg_perp_order<'c: 'info, 'info>(
        ctx: Context<'_, '_, 'c, 'info, PlaceAndMakeSignedMsg<'info>>,
        signed_msg_order_uuid: [u8; 8],
    ) -> Result<()> {
        handle_place_and_make_signed_msg_perp_order(ctx, signed_msg_order_uuid)
    }
    // --------- End: (AccountLoader & AccountInfo) load State case ---------



    // --------- Start: Instruction 64 Accounts Limitations && ReaminingAccount analysis ---------

    // Instruction 64 Accounts Limitations && ReaminingAccount analysis
    //  scenario 1: entrypoint instruction can bring more than 64 accounts by reamaining accounts, but not use it,
    //  scenario 2: for CPI case, we subtract it by demand and also can't more than 64 accounts.
    //  scenario 3: watch account lifetime.
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
    // --------- End: Instruction 64 Accounts Limitations && ReaminingAccount analysis ---------



    // multiple signers case:
    pub fn multiple_signers_demo(ctx: Context<MultipleSignersAccounts>) -> Result<()> {
        instructions::multiple_signers_demo(ctx)
    }

}
