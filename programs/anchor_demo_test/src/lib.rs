use anchor_lang::prelude::*;

declare_id!("BhuwMLdcx6Ux3cia4SyzPk3DprpDhjYBdqcEKjF2UQHr");

#[program]
pub mod anchor_demo_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
