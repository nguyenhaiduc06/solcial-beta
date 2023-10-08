use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Like {
    pub user: Pubkey,
    pub post: Pubkey,
    pub bump: u8,
}

const USER_SPACE: usize = PUBKEY_SPACE;
const POST_SPACE: usize = PUBKEY_SPACE;
const BUMP_SPACE: usize = 1;

impl Like {
    const SPACE: usize = DISCRIMINATOR_SPACE + USER_SPACE + POST_SPACE + BUMP_SPACE;
}

#[derive(Accounts)]
#[instruction(post: Pubkey)]
pub struct CreateLike<'info> {
    #[account(init, payer = user, space = Like::SPACE, seeds = [b"like", user.key().as_ref(), post.key().as_ref()], bump)]
    pub like: Account<'info, Like>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteLike<'info> {
    #[account(mut, close = user, seeds = [b"like", user.key().as_ref(), like.post.key().as_ref()], bump = like.bump)]
    pub like: Account<'info, Like>,
    pub user: Signer<'info>,
}
