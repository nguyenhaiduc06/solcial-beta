use anchor_lang::prelude::*;

use super::*;

#[account]
pub struct Profile {
    pub name: String,
    pub username: String,
    pub bio: String,
    pub bump: u8,
}

pub const MAX_NAME_CHAR_COUNT: usize = 20;
pub const MAX_USERNAME_CHAR_COUNT: usize = 20;
pub const MAX_BIO_CHAR_COUNT: usize = 50;

const MAX_NAME_LENGTH: usize = MAX_NAME_CHAR_COUNT * 4;
const MAX_USERNAME_LENGTH: usize = MAX_USERNAME_CHAR_COUNT * 4;
const MAX_BIO_LENGTH: usize = MAX_BIO_CHAR_COUNT * 4;
const BUMP_LENGTH: usize = 1;

impl Profile {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + STRING_LENGTH_PREFIX
        + MAX_NAME_LENGTH
        + STRING_LENGTH_PREFIX
        + MAX_USERNAME_LENGTH
        + STRING_LENGTH_PREFIX
        + MAX_BIO_LENGTH
        + BUMP_LENGTH;
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer = user, space = Profile::LEN, seeds = [b"profile", user.key().as_ref()], bump)]
    pub profile: Account<'info, Profile>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"profile", user.key().as_ref()], bump = profile.bump)]
    pub profile: Account<'info, Profile>,
}

#[derive(Accounts)]
pub struct DeleteProfile<'info> {
    pub user: Signer<'info>,
    #[account(mut, close = user, seeds = [b"profile", user.key().as_ref()], bump = profile.bump)]
    pub profile: Account<'info, Profile>,
}
