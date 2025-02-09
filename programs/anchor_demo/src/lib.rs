pub mod adapters;
pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;
pub mod utils;

use anchor_lang::prelude::*;
pub use constants::*;
use instructions::*;
use states::*;

declare_id!("2vrz1hEC3HpVxPNXACwy4oMbVqPEw8sA34UYr6KLwQpL");

#[program]
pub mod anchor_demo {
    use super::*;
    use chrono::*; // ne

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        // msg!("Test Output");

        let clock: Clock = Clock::get()?;
        msg!(
            "Block timestamp: {}",
            // Get block.timestamp
            clock.unix_timestamp,
        );

        let time_stamp = clock.unix_timestamp; // current timestamp
        let date_time = chrono::NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
        let day_of_the_week = date_time.weekday();
        msg!("day_of_the_week: {}", day_of_the_week,);
        emit!(MyEvent { amount_in: 42 });

        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;

        msg!("The signer1: {:?}", *the_signer1.key);
        msg!("The signer2: {:?}", *the_signer2.key);

        Ok(())
    }

    pub fn check_identity(ctx: Context<OnlyOwner>) -> Result<()> {
        // Function logic...
        msg!("Hello, I'm the owner.");
        instructions::check_identity(ctx)
    }

    // example method:
    //  to record commom methods about Program/Anchor implementation
    pub fn commission_sol_proxy_swap<'info>(
        ctx: Context<CommissionSOLProxySwapAccounts<'info>>,
        data: SwapArgs,
        commission_rate: u16,
        commission_direction: bool,
        order_id: u64,
    ) -> Result<u64> {
        instructions::commission_sol_proxy_swap_handler(
            ctx,
            data,
            commission_rate,
            commission_direction,
            order_id,
        )
    }
}
