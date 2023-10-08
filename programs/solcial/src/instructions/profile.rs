use crate::state::profile::*;
use anchor_lang::prelude::*;

pub fn create_profile(ctx: Context<CreateProfile>, username: String) -> Result<()> {
    let profile = &mut ctx.accounts.profile;

    profile.username = username;
    profile.bump = *ctx.bumps.get("profile").unwrap();

    Ok(())
}

pub fn update_profile(ctx: Context<UpdateProfile>, new_username: String) -> Result<()> {
    let profile = &mut ctx.accounts.profile;

    profile.username = new_username;

    Ok(())
}

pub fn delete_profile(_ctx: Context<DeleteProfile>) -> Result<()> {
    Ok(())
}
