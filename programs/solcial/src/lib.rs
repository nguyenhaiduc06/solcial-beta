use anchor_lang::prelude::*;

declare_id!("Z85hXczHvADAyPsa56QW1WUwhu1bUJbo6sQaaFhM1Fy");

#[program]
pub mod solcial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
