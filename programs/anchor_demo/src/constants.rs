use anchor_lang::prelude::*;

pub const MOCK_OWNER: &str = "GTx4x6mqoNydgvVUvHfhenTCoCyhdKZ9gzYtY4zRMLzf";


#[constant]
pub const SEED_SA: &[u8] = b"sa";
pub const BUMP_SA: u8 = 251;
pub const COMMISSION_RATE_LIMIT: u16 = 300;
pub const COMMISSION_DENOMINATOR: u64 = 10000;
pub const MAX_HOPS: usize = 3;
pub const TOTAL_WEIGHT: u8 = 100;

pub const SWAP_SELECTOR: &[u8; 8] = &[248, 198, 158, 145, 225, 117, 135, 200];
pub const CPSWAP_SELECTOR: &[u8; 8] = &[143, 190, 90, 218, 196, 30, 51, 222];
pub const SWAP_V2_SELECTOR: &[u8; 8] = &[43, 4, 237, 11, 26, 201, 30, 98];
pub const PLACE_TAKE_ORDER_SELECTOR: &[u8; 8] = &[3, 44, 71, 3, 26, 199, 203, 85];
pub const BRIDGE_TO_LOG_SELECTOR: &[u8; 8] = &[212, 189, 176, 218, 196, 135, 64, 122];
pub const ZERO_ADDRESS: Pubkey = Pubkey::new_from_array([0u8; 32]);

pub const PUMPFUN_BUY_SELECTOR: &[u8; 8] = &[102, 6, 61, 18, 1, 218, 235, 234];
pub const PUMPFUN_SELL_SELECTOR: &[u8; 8] = &[51, 230, 133, 164, 1, 127, 131, 173];

pub const SIGNED_MSG_PDA_SEED: &[u8] = b"signed_msg_pda_seed";


pub mod authority_pda {
    use anchor_lang::declare_id;
    declare_id!("So11111111111111111111111111111111111111112"); // TODO
    // declare_id!("So11111111111111111111111111111111111111112"); //pre_deploy
}
pub mod wsol_program {
    use anchor_lang::declare_id;
    declare_id!("So11111111111111111111111111111111111111112");
}

pub const X_AUTHORITY: &[u8] = b"x_authority";
pub const SEED_AUTH_STORE: &[u8] = b"auth_store";


