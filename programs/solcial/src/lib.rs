use anchor_lang::prelude::*;
use state::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("Z85hXczHvADAyPsa56QW1WUwhu1bUJbo6sQaaFhM1Fy");

#[program]
pub mod solcial {
    use super::*;

    pub fn create_profile(
        ctx: Context<CreateProfile>,
        name: String,
        username: String,
        bio: String,
    ) -> Result<()> {
        instructions::create_profile(ctx, name, username, bio)
    }

    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        new_name: String,
        new_username: String,
        new_bio: String,
    ) -> Result<()> {
        instructions::update_profile(ctx, new_name, new_username, new_bio)
    }

    pub fn delete_profile(ctx: Context<DeleteProfile>) -> Result<()> {
        instructions::delete_profile(ctx)
    }

    pub fn create_post(ctx: Context<CreatePost>, content: String, tag: String) -> Result<()> {
        instructions::create_post(ctx, content, tag)
    }
}
