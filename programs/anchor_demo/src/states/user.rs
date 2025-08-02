use std::num::NonZeroI64;

use anchor_lang::prelude::*;

use crate::states::traits::Size;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, AnchorSerialize, AnchorDeserialize)]
#[repr(transparent)]
pub struct Price(pub i64);  // Changed from NonZeroI64 to i64 for Anchor compatibility
// pub struct Price(pub NonZeroI64);


impl Size for User {
    const SIZE: usize = 4376;
}

#[account(zero_copy(unsafe))]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct User {
    /// The owner/authority of the account
    pub authority: Pubkey,
    /// An addresses that can control the account on the authority's behalf. Has limited power, cant withdraw
    pub delegate: Pubkey,
    /// Encoded display name e.g. "toly"
    pub name: [u8; 32],

    /// The last time the user added perp lp positions
    pub last_add_perp_lp_shares_ts: i64,
    /// The total values of deposits the user has made
    /// precision: QUOTE_PRECISION
    pub total_deposits: u64,
    /// The total values of withdrawals the user has made
    /// precision: QUOTE_PRECISION
    pub total_withdraws: u64,
    /// The total socialized loss the users has incurred upon the protocol
    /// precision: QUOTE_PRECISION
    pub total_social_loss: u64,
    /// Fees (taker fees, maker rebate, referrer reward, filler reward) and pnl for perps
    /// precision: QUOTE_PRECISION
    pub settled_perp_pnl: i64,
    /// Fees (taker fees, maker rebate, filler reward) for spot
    /// precision: QUOTE_PRECISION
    pub cumulative_spot_fees: i64,
    /// Cumulative funding paid/received for perps
    /// precision: QUOTE_PRECISION
    pub cumulative_perp_funding: i64,
    /// The amount of margin freed during liquidation. Used to force the liquidation to occur over a period of time
    /// Defaults to zero when not being liquidated
    /// precision: QUOTE_PRECISION
    pub liquidation_margin_freed: u64,
    /// The last slot a user was active. Used to determine if a user is idle
    pub last_active_slot: u64,
    /// Every user order has an order id. This is the next order id to be used
    pub next_order_id: u32,
    /// Custom max initial margin ratio for the user
    pub max_margin_ratio: u32,
    /// The next liquidation id to be used for user
    pub next_liquidation_id: u16,
    /// The sub account id for this user
    pub sub_account_id: u16,
    /// Whether the user is active, being liquidated or bankrupt
    pub status: u8,
    /// Whether the user has enabled margin trading
    pub is_margin_trading_enabled: bool,
    /// User is idle if they haven't interacted with the protocol in 1 week and they have no orders, perp positions or borrows
    /// Off-chain keeper bots can ignore users that are idle
    pub idle: bool,
    /// number of open orders
    pub open_orders: u8,
    /// Whether or not user has open order
    pub has_open_order: bool,
    /// number of open orders with auction
    pub open_auctions: u8,
    /// Whether or not user has open order with auction
    pub has_open_auction: bool,
    pub pool_id: u8,
    pub padding1: [u8; 3],
    pub last_fuel_bonus_update_ts: u32,
    pub padding: [u8; 12],
}

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
#[derive(Default)]
pub struct UserStats {
    /// The authority for all of a users sub accounts
    pub authority: Pubkey,
    /// The address that referred this user
    pub referrer: Pubkey,

    /// The timestamp of the next epoch
    /// Epoch is used to limit referrer rewards earned in single epoch
    pub next_epoch_ts: i64,

    /// Rolling 30day maker volume for user
    /// precision: QUOTE_PRECISION
    pub maker_volume_30d: u64,
    /// Rolling 30day taker volume for user
    /// precision: QUOTE_PRECISION
    pub taker_volume_30d: u64,
    /// Rolling 30day filler volume for user
    /// precision: QUOTE_PRECISION
    pub filler_volume_30d: u64,
    /// last time the maker volume was updated
    pub last_maker_volume_30d_ts: i64,
    /// last time the taker volume was updated
    pub last_taker_volume_30d_ts: i64,
    /// last time the filler volume was updated
    pub last_filler_volume_30d_ts: i64,

    /// The amount of tokens staked in the quote spot markets if
    pub if_staked_quote_asset_amount: u64,
    /// The current number of sub accounts
    pub number_of_sub_accounts: u16,
    /// The number of sub accounts created. Can be greater than the number of sub accounts if user
    /// has deleted sub accounts
    pub number_of_sub_accounts_created: u16,
    /// Flags for referrer status:
    /// First bit (LSB): 1 if user is a referrer, 0 otherwise
    /// Second bit: 1 if user was referred, 0 otherwise
    pub referrer_status: u8,
    pub disable_update_perp_bid_ask_twap: bool,
    pub padding1: [u8; 1],
    /// whether the user has a FuelOverflow account
    pub fuel_overflow_status: u8,
    /// accumulated fuel for token amounts of insurance
    pub fuel_insurance: u32,
    /// accumulated fuel for notional of deposits
    pub fuel_deposits: u32,
    /// accumulate fuel bonus for notional of borrows
    pub fuel_borrows: u32,
    /// accumulated fuel for perp open interest
    pub fuel_positions: u32,
    /// accumulate fuel bonus for taker volume
    pub fuel_taker: u32,
    /// accumulate fuel bonus for maker volume
    pub fuel_maker: u32,

    /// The amount of tokens staked in the governance spot markets if
    pub if_staked_gov_token_amount: u64,

    /// last unix ts user stats data was used to update if fuel (u32 to save space)
    pub last_fuel_if_bonus_update_ts: u32,

    pub padding: [u8; 12],
}

impl Size for UserStats {
    const SIZE: usize = 240;
}