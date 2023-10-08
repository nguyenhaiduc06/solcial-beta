use crate::errors::ErrorCode;
use crate::state::profile::*;
use anchor_lang::prelude::*;

pub fn create_profile(
    ctx: Context<CreateProfile>,
    name: String,
    username: String,
    bio: String,
) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    require!(
        name.chars().count() <= MAX_NAME_CHAR_COUNT,
        ErrorCode::TooLong
    );
    require!(
        username.chars().count() <= MAX_USERNAME_CHAR_COUNT,
        ErrorCode::TooLong
    );
    require!(
        bio.chars().count() <= MAX_BIO_CHAR_COUNT,
        ErrorCode::TooLong
    );

    profile.name = name;
    profile.username = username;
    profile.bio = bio;
    profile.bump = *ctx.bumps.get("profile").unwrap();

    Ok(())
}

pub fn update_profile(
    ctx: Context<UpdateProfile>,
    new_name: String,
    new_username: String,
    new_bio: String,
) -> Result<()> {
    let profile = &mut ctx.accounts.profile;

    require!(
        new_name.chars().count() <= MAX_NAME_CHAR_COUNT,
        ErrorCode::TooLong
    );
    require!(
        new_username.chars().count() <= MAX_USERNAME_CHAR_COUNT,
        ErrorCode::TooLong
    );
    require!(
        new_bio.chars().count() <= MAX_BIO_CHAR_COUNT,
        ErrorCode::TooLong
    );

    profile.name = new_name;
    profile.username = new_username;
    profile.bio = new_bio;

    Ok(())
}

pub fn delete_profile(_ctx: Context<DeleteProfile>) -> Result<()> {
    Ok(())
}
