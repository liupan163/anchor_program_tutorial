use anchor_lang::prelude::*;

use crate::instructions::Dex;



#[event]
#[derive(Debug)]
pub struct MyEvent {
    pub amount_in: u64,
}

#[event]
#[derive(Debug)]
pub struct SwapEvent {
    pub dex: Dex,
    pub amount_in: u64,
    pub amount_out: u64,
}
