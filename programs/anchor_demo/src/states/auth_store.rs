use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[derive(Debug)]
#[repr(C)]
pub struct AuthStore {
    pub admin: Pubkey,

    pub xxx_address: Pubkey,
    pub xxx_list: [OperatorWallet; 50],

    pub is_ok: u8,
    pub signer_wallet_list: [Pubkey; 10],
    pub signer_threshold: u8,
}

#[zero_copy(unsafe)]
#[repr(C)]
#[derive(Default, Eq, PartialEq, Debug)]
pub struct OperatorWallet {
    pub wallet_pubkey: Pubkey,
    pub x_mark: u64,
    pub y_mark: u64,
}
