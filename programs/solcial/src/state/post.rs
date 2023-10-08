use anchor_lang::prelude::*;

use super::{DISCRIMINATOR_SPACE, PUBKEY_SPACE, STRING_LENGTH_PREFIX_SPACE};

#[account]
pub struct Post {
    pub content: String,
    pub tag: String,
    pub author: Pubkey,
    pub timestamp: i64,
}

pub const MAX_CONTENT_CHAR_COUNT: usize = 200;
pub const MAX_TAG_CHAR_COUNT: usize = 50;

const CONTENT_SPACE: usize = STRING_LENGTH_PREFIX_SPACE + MAX_CONTENT_CHAR_COUNT * 4;
const TAG_SPACE: usize = STRING_LENGTH_PREFIX_SPACE + MAX_TAG_CHAR_COUNT * 4;
const AUTHOR_SPACE: usize = PUBKEY_SPACE;
const TIMESTAMP_SPACE: usize = 8;

impl Post {
    const SPACE: usize =
        DISCRIMINATOR_SPACE + CONTENT_SPACE + TAG_SPACE + AUTHOR_SPACE + TIMESTAMP_SPACE;
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = author, space = Post::SPACE)]
    pub post: Account<'info, Post>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdatePost<'info> {
    #[account(mut)]
    pub post: Account<'info, Post>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeletePost<'info> {
    #[account(mut, has_one = author)]
    pub post: Account<'info, Post>,
    pub author: Signer<'info>,
}
