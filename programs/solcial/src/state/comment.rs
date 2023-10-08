use anchor_lang::prelude::*;

use super::{DISCRIMINATOR_SPACE, PUBKEY_SPACE, STRING_LENGTH_PREFIX_SPACE};

#[account]
pub struct Comment {
    pub author: Pubkey,
    pub post: Pubkey,
    pub content: String,
}

pub const MAX_COMMENT_CONTENT_CHAR_COUNT: usize = 200;

const AUTHOR_SPACE: usize = PUBKEY_SPACE;
const POST_SPACE: usize = PUBKEY_SPACE;
const CONTENT_SPACE: usize = STRING_LENGTH_PREFIX_SPACE + MAX_COMMENT_CONTENT_CHAR_COUNT * 4;

impl Comment {
    const SPACE: usize = DISCRIMINATOR_SPACE + AUTHOR_SPACE + POST_SPACE + CONTENT_SPACE;
}

#[derive(Accounts)]
pub struct CreateComment<'info> {
    #[account(init, payer = author, space = Comment::SPACE)]
    pub comment: Account<'info, Comment>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateComment<'info> {
    #[account(mut)]
    pub comment: Account<'info, Comment>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteComment<'info> {
    #[account(mut, close = author, has_one = author)]
    pub comment: Account<'info, Comment>,
    pub author: Signer<'info>,
}
