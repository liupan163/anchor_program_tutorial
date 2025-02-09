
use anchor_lang::prelude::*;



#[event]
#[derive(Debug)]
pub struct MyEvent {
    pub amount_in: u64,
}
