use crate::errors::ErrorCode;
use crate::state::post::*;
use anchor_lang::prelude::*;

pub fn create_post(ctx: Context<CreatePost>, content: String, tag: String) -> Result<()> {
    let post = &mut ctx.accounts.post;
    let author = &ctx.accounts.author;
    let clock = Clock::get().unwrap();

    require!(
        content.chars().count() <= MAX_CONTENT_CHAR_COUNT,
        ErrorCode::TooLong
    );
    require!(
        tag.chars().count() <= MAX_TAG_CHAR_COUNT,
        ErrorCode::TooLong
    );

    post.content = content;
    post.tag = tag.to_lowercase();
    post.author = *author.key;
    post.timestamp = clock.unix_timestamp;

    Ok(())
}

pub fn update_post() -> Result<()> {
    Ok(())
}

pub fn delete_post() -> Result<()> {
    Ok(())
}
