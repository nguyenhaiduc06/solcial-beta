use anchor_lang::prelude::*;
use state::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("Z85hXczHvADAyPsa56QW1WUwhu1bUJbo6sQaaFhM1Fy");

#[program]
pub mod solcial {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, username: String) -> Result<()> {
        instructions::create_profile(ctx, username)
    }

    pub fn update_profile(ctx: Context<UpdateProfile>, new_username: String) -> Result<()> {
        instructions::update_profile(ctx, new_username)
    }

    pub fn delete_profile(ctx: Context<DeleteProfile>) -> Result<()> {
        instructions::delete_profile(ctx)
    }
}
