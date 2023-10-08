use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::comment::*;

pub fn create_comment(ctx: Context<CreateComment>, content: String, post: Pubkey) -> Result<()> {
    let comment = &mut ctx.accounts.comment;
    let author = &ctx.accounts.author;

    require!(
        content.chars().count() <= MAX_COMMENT_CONTENT_CHAR_COUNT,
        ErrorCode::TooLong
    );

    comment.content = content;
    comment.author = *author.key;
    comment.post = post;

    Ok(())
}

pub fn udpate_comment(ctx: Context<UpdateComment>, new_content: String) -> Result<()> {
    let comment = &mut ctx.accounts.comment;

    require!(
        new_content.chars().count() <= MAX_COMMENT_CONTENT_CHAR_COUNT,
        ErrorCode::TooLong
    );

    comment.content = new_content;

    Ok(())
}

pub fn delete_comment(_ctx: Context<DeleteComment>) -> Result<()> {
    Ok(())
}
