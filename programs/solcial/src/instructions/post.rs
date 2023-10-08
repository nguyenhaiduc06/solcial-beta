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

pub fn update_post(ctx: Context<UpdatePost>, new_content: String, new_tag: String) -> Result<()> {
    let post = &mut ctx.accounts.post;

    require!(
        new_content.chars().count() <= MAX_CONTENT_CHAR_COUNT,
        ErrorCode::TooLong
    );
    require!(
        new_tag.chars().count() <= MAX_TAG_CHAR_COUNT,
        ErrorCode::TooLong
    );

    post.content = new_content;
    post.tag = new_tag.to_lowercase();

    Ok(())
}

pub fn delete_post(_ctx: Context<DeletePost>) -> Result<()> {
    Ok(())
}
