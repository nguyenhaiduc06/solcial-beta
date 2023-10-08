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

    pub fn update_post(
        ctx: Context<UpdatePost>,
        new_content: String,
        new_tag: String,
    ) -> Result<()> {
        instructions::update_post(ctx, new_content, new_tag)
    }

    pub fn delete_post(ctx: Context<DeletePost>) -> Result<()> {
        instructions::delete_post(ctx)
    }

    pub fn create_comment(
        ctx: Context<CreateComment>,
        content: String,
        post: Pubkey,
    ) -> Result<()> {
        instructions::create_comment(ctx, content, post)
    }

    pub fn update_comment(ctx: Context<UpdateComment>, new_content: String) -> Result<()> {
        instructions::udpate_comment(ctx, new_content)
    }

    pub fn delete_comment(ctx: Context<DeleteComment>) -> Result<()> {
        instructions::delete_comment(ctx)
    }
}
