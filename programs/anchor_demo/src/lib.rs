use anchor_lang::prelude::*;

declare_id!("6Ps266QnytvWzjEHttuhtGUHwKFDp98xvj8oqFHo5V3x");

// HQiNbppppSq2HEaXDjfnBHbmmmnbho3mjCvn5FEEsTdr
// 6Ps266QnytvWzjEHttuhtGUHwKFDp98xvj8oqFHo5V3x
// 9xezg88WgR7RYkWV9LSa3NSsQaJTDPeVtUksajpmmoP4
// 6wMgMx9BrfpTqE9vFmtWU6bJi8JX6iGmFXUFM2zm6MbD
// 4PvZJAqSCEL2fJ1ssT3jwp2EGbzCN7BDooyy4MBxkkJW
// HnWGvRLFS2P6wNxAuGW8Kmm3T1e6ztvKBJNBTnhPcP4i

// NOTE: Replace with your wallet's public key
const OWNER: &str = "GTx4x6mqoNydgvVUvHfhenTCoCyhdKZ9gzYtY4zRMLzf";

#[program]
pub mod anchor_demo {
    use super::*;
    use anchor_lang::prelude::*;
    use chrono::*;  // ne

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

        emit!(MyEvent { value: 42 });

        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;

        msg!("The signer1: {:?}", *the_signer1.key);
        msg!("The signer2: {:?}", *the_signer2.key);

        Ok(())
    }

    pub fn limit_range(_ctx: Context<LimitRange>, a: u64) -> Result<()> {
        /* if a < 10 {
            return err!(MyError::AisTooSmall);
        }
        if a > 100 {
            return err!(MyError::AisTooBig);
        } */

        require!(a >= 10, MyError::AisTooSmall);
        require!(a <= 100, MyError::AisTooBig);

        msg!("Result = {}", a);

        Ok(())
    }

    #[access_control(check(&ctx))]
    pub fn checkIdentity(ctx: Context<OnlyOwner>) -> Result<()> {
        // Function logic...

        msg!("Holla, I'm the owner.");
        Ok(())
    }
}
fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    // Check if signer === owner
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );

    Ok(())
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

// An enum for custom error codes
#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
}

#[derive(Accounts)]
pub struct LimitRange {}

#[error_code]
pub enum MyError {
    #[msg("a is too big")]
    AisTooBig,
    #[msg("a is too small")]
    AisTooSmall,
}

#[event]
pub struct MyEvent {
    pub value: u64,
}