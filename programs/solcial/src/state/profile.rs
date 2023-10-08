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

const NAME_SPACE: usize = STRING_LENGTH_PREFIX_SPACE + MAX_NAME_CHAR_COUNT * 4;
const USERNAME_SPACE: usize = STRING_LENGTH_PREFIX_SPACE + MAX_USERNAME_CHAR_COUNT * 4;
const BIO_SPACE: usize = STRING_LENGTH_PREFIX_SPACE + MAX_BIO_CHAR_COUNT * 4;
const BUMP_SPACE: usize = 1;

impl Profile {
    const SPACE: usize = DISCRIMINATOR_SPACE + NAME_SPACE + USERNAME_SPACE + BIO_SPACE + BUMP_SPACE;
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer = user, space = Profile::SPACE, seeds = [b"profile", user.key().as_ref()], bump)]
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
